use crate::args::Args;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use crate::common::ResourceType;

fn mangle_path(path: &str) -> String {
    let mut mangled = String::from("AssetDir_");
    for c in path.chars() {
        let safe_ranges = ['a'..='z', 'A'..='Z', '0'..='9'];
        let safe = safe_ranges.iter().any(|range| range.contains(&c));
        if safe {
            mangled.push(c);
        } else {
            mangled.push('_');
            mangled.push_str(&mut (c as u32).to_string());
            mangled.push('x');
        }
    }
    mangled
}

fn make_name_safe(name: &str) -> String {
    let mut safe_name = String::with_capacity(name.len());
    let mut chars = name.chars();
    let first_char = chars.next().expect("Resource or directory name is empty");
    if !('a'..='z').contains(&first_char) {
        panic!("Invalid resource or directory name");
    }
    safe_name.push(first_char);

    for c in chars {
        if ('a'..='z').contains(&c) {
            safe_name.push(c);
        } else if c == '-' {
            safe_name.push('_');
        } else if c == '_' {
            panic!("Resource or directory name contains underscores. Replace them with dashes");
        } else {
            panic!("Invalid resource or directory name");
        }
    }
    safe_name
}

#[derive(Debug, Clone)]
struct Resource {
    pub resource_type: ResourceType,
    pub constructor_code: TokenStream,
}

impl Resource {
    fn codegen(&self) -> TokenStream {
        self.constructor_code.clone()
    }
}

#[derive(Debug, Clone, Default)]
struct DirectoryStruct {
    pub subdirectories: HashMap<String, DirectoryStruct>,
    pub resources: HashMap<String, Resource>,
}

impl DirectoryStruct {
    pub fn new() -> Self {
        Self::default()
    }

    fn prefixed_codegen(&self, prefix: &str) -> TokenStream {
        let struct_name = mangle_path(prefix);
        let mut resources: Vec<_> = self.resources.iter().collect();
        resources.sort_by_key(|&(name, _res)| name);

        let resource_fields = resources
            .iter()
            .map(|&(name, _res)| format!("_res_{}", make_name_safe(name)));

        let resource_types = resources.iter().map(
            |&(_name, res)| res.resource_type.rust_type());

        let subdir_tokens = self.subdirectories.iter().map(|(name, subdir)| {
            subdir.prefixed_codegen(&format!("{}/{}", prefix, name))
        });

        let tokens = quote! {
            pub struct #struct_name {
                #(pub #resource_fields: ::std::option::Option<#resource_types>,)*
            }
            #(#subdir_tokens)*
        };
        tokens.into()
    }

    pub fn codegen(&self) -> TokenStream {
        self.prefixed_codegen("root")
    }
}

fn add_resource_by_path(
    path_components: &[&str],
    resource: Resource,
    target: &mut DirectoryStruct,
) {
    if let &[leaf_component] = path_components {
        if let Some(_) = target
            .resources
            .insert(String::from(leaf_component), resource)
        {
            panic!("Duplicate resource {:?}", path_components);
        }
    } else {
        if let Some((&head, tail)) = path_components.split_first() {
            let subdir = target
                .subdirectories
                .entry(String::from(head))
                .or_insert_with(|| DirectoryStruct::new());
            add_resource_by_path(tail, resource, subdir);
        }
    }
}

fn build_resource(path: &str, resource_type: ResourceType) -> Resource {
    let args = Args {
        path: String::from(path),
    };
    let constructor_code = match resource_type {
        ResourceType::PassMap => crate::pass_map::load_pass_map(&args),
        ResourceType::Room => crate::room::load_room(&args),
        ResourceType::RustScript => crate::rust_script::load_rust_script(&args),
        ResourceType::Texture => crate::texture::load_texture(&args),
    };
    Resource {
        resource_type,
        constructor_code: constructor_code.into(),
    }
}

pub fn generate_resource_storage_code<'a, I>(resources: I) -> TokenStream
where
    I: IntoIterator<Item = (&'a str, ResourceType)>,
{
    let mut root = DirectoryStruct::new();
    for (path, resource_type) in resources {
        let path_components: Vec<_> = path.split('/').collect();
        let resource = build_resource(path, resource_type);
        add_resource_by_path(&path_components, resource, &mut root);
    }

    root.codegen()
}
