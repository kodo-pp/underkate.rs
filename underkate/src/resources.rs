use crate::graphics::texture::Texture;
use ggez::Context;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use underkate_tools::load_texture;

#[derive(Debug, Copy, Clone)]
pub struct ResourceDoesNotExist<'a> {
    pub name: &'a str,
}

impl Display for ResourceDoesNotExist<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "Resource with name `{}` does not exist", self.name)
    }
}

impl Error for ResourceDoesNotExist<'_> {}

pub trait ResourceStorage<T> {
    fn try_get<'a>(&self, name: &'a str) -> Result<&T, ResourceDoesNotExist<'a>>;
    fn put(&mut self, name: String, resource: T);

    fn get(&self, name: &str) -> &T {
        self.try_get(name).unwrap()
    }
}

pub struct GlobalResourceStorage {
    textures: HashMap<String, Texture>,
}

impl GlobalResourceStorage {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }
}

impl ResourceStorage<Texture> for GlobalResourceStorage {
    fn try_get<'a>(&self, name: &'a str) -> Result<&Texture, ResourceDoesNotExist<'a>> {
        self.textures.get(name).ok_or(ResourceDoesNotExist { name })
    }

    fn put(&mut self, name: String, resource: Texture) {
        match self.textures.insert(name, resource) {
            Some(_) => panic!("Duplicate resource name"),
            None => (),
        }
    }
}

macro_rules! use_texture {
    ($path:tt => $storage:expr, $ctx:expr) => {
        let ctx: &mut Context = $ctx;
        $storage.put(String::from($path), load_texture!($path));
    };
}

pub fn make_global_storage(ctx: &mut Context) -> GlobalResourceStorage {
    let mut storage = GlobalResourceStorage::new();
    use_texture!("overworld/player/front" => storage, ctx);
    use_texture!("overworld/player/back" => storage, ctx);
    use_texture!("overworld/player/leftward" => storage, ctx);
    use_texture!("overworld/player/rightward" => storage, ctx);
    use_texture!("overworld/rooms/_stub/bg" => storage, ctx);
    storage
}
