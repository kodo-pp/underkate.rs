use crate::graphics::texture::Texture;
use crate::overworld::room::PartialCreationParams as RoomPartialCreationParams;
use ggez::Context;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use underkate_tools::{load_room, load_texture};

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
}

impl GlobalResourceStorage {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            room_partial_creation_params: HashMap::new(),
        }
    }
}

impl ResourceStorage<Texture> for GlobalResourceStorage {
    fn try_get<'a>(&self, name: &'a str) -> Result<&Texture, ResourceDoesNotExist<'a>> {
        self.textures.get(name).ok_or(ResourceDoesNotExist { name })
    }

    fn put(&mut self, name: String, resource: Texture) {
        if let Some(_) = self.textures.insert(name, resource) {
            panic!("Duplicate resource name");
        }
    }
}

impl ResourceStorage<RoomPartialCreationParams> for GlobalResourceStorage {
    fn try_get<'a>(
        &self,
        name: &'a str,
    ) -> Result<&RoomPartialCreationParams, ResourceDoesNotExist<'a>> {
        self.room_partial_creation_params
            .get(name)
            .ok_or(ResourceDoesNotExist { name })
    }

    fn put(&mut self, name: String, resource: RoomPartialCreationParams) {
        if let Some(_) = self.room_partial_creation_params.insert(name, resource) {
            panic!("Duplicate resource name");
        }
    }
}

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

pub fn make_global_storage(ctx: &mut Context) -> GlobalResourceStorage {
    let mut storage = GlobalResourceStorage::new();
    use_texture!("overworld/player/front" => storage, ctx);
    use_texture!("overworld/player/back" => storage, ctx);
    use_texture!("overworld/player/leftward" => storage, ctx);
    use_texture!("overworld/player/rightward" => storage, ctx);
    use_texture!("overworld/rooms/home/room/bg" => storage, ctx);
    use_texture!("overworld/rooms/home/room/pass" => storage, ctx);
    use_room!("home/room" => storage);
    storage
}
