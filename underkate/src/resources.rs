use crate::graphics::texture::Texture;
use crate::overworld::pass_map::BitmapPassMap;
use crate::overworld::room::PartialCreationParams as RoomPartialCreationParams;
use ggez::Context;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use underkate_tools::{load_room, load_texture, load_pass_map, load_rust_script};
use crate::script::rust_script::RustScript;

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

pub trait ResourceStorageCloneExt<T: Clone>: ResourceStorage<T> {
    fn try_get_cloned<'a>(&self, name: &'a str) -> Result<T, ResourceDoesNotExist<'a>>;
    fn get_cloned(&self, name: &str) -> T;
}

impl<T: Clone, S: ResourceStorage<T>> ResourceStorageCloneExt<T> for S {
    fn try_get_cloned<'a>(&self, name: &'a str) -> Result<T, ResourceDoesNotExist<'a>> {
        self.try_get(name).map(Clone::clone)
    }

    fn get_cloned(&self, name: &str) -> T {
        self.try_get_cloned(name).unwrap()
    }
}

pub struct GlobalResourceStorage {
    textures: HashMap<String, Texture>,
    room_partial_creation_params: HashMap<String, RoomPartialCreationParams>,
    bitmap_pass_maps: HashMap<String, BitmapPassMap>,
    rust_scripts: HashMap<String, RustScript>,
}

impl GlobalResourceStorage {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            room_partial_creation_params: HashMap::new(),
            bitmap_pass_maps: HashMap::new(),
            rust_scripts: HashMap::new(),
        }
    }
}

macro_rules! resource_storage_impl {
    ($resource:ty as .$field:ident) => {
        impl ResourceStorage<$resource> for GlobalResourceStorage {
            fn try_get<'a>(&self, name: &'a str) -> Result<&$resource, ResourceDoesNotExist<'a>> {
                self.$field.get(name).ok_or(ResourceDoesNotExist { name })
            }

            fn put(&mut self, name: String, resource: $resource) {
                if let Some(_) = self.$field.insert(name, resource) {
                    panic!("Duplicate resource name");
                }
            }
        }
    };
}

resource_storage_impl!(Texture as .textures);
resource_storage_impl!(RoomPartialCreationParams as .room_partial_creation_params);
resource_storage_impl!(BitmapPassMap as .bitmap_pass_maps);
resource_storage_impl!(RustScript as .rust_scripts);

macro_rules! use_texture {
    ($path:tt => $storage:expr, $ctx:expr) => {
        let ctx: &mut Context = $ctx;
        $storage.put(String::from($path), load_texture!($path));
    };
}

macro_rules! use_room {
    ($path:tt => $storage:expr) => {
        $storage.put(String::from($path), load_room!($path));
    };
}

macro_rules! use_pass_map {
    ($path:tt => $storage:expr) => {
        $storage.put(String::from($path), load_pass_map!($path));
    };
}

macro_rules! use_rust_script {
    ($path:tt => $storage:expr) => {
        $storage.put(String::from($path), load_rust_script!($path));
    };
}

pub fn make_global_storage(ctx: &mut Context) -> GlobalResourceStorage {
    let mut storage = GlobalResourceStorage::new();
    use_texture!("overworld/player/front" => storage, ctx);
    use_texture!("overworld/player/back" => storage, ctx);
    use_texture!("overworld/player/leftward" => storage, ctx);
    use_texture!("overworld/player/rightward" => storage, ctx);
    use_texture!("overworld/rooms/home/room/bg" => storage, ctx);
    use_pass_map!("overworld/rooms/home/room" => storage);
    use_room!("home/room" => storage);
    use_rust_script!("overworld/rooms/home/room/init" => storage);

    storage
}
