use euclid::{Box2D, Length, Point2D, Size2D, Vector2D};
use ggez::mint;

pub mod unit {
    pub struct Overworld;
    pub struct Screen;
}

macro_rules! make_typedef {
    ($euclid_name:ident => $underkate_name:ident for $unit:ty) => {
        #[allow(dead_code)]
        pub type $underkate_name<T> = $euclid_name<T, $unit>;
    };
}

use unit::{Overworld, Screen};

make_typedef!(Point2D => OverworldPoint for Overworld);
make_typedef!(Point2D => ScreenPoint for Screen);

make_typedef!(Size2D => OverworldDimensions for Overworld);
make_typedef!(Size2D => ScreenDimensions for Screen);

make_typedef!(Box2D => OverworldRect for Overworld);
make_typedef!(Box2D => ScreenRect for Screen);

make_typedef!(Length => OverworldLength for Overworld);
make_typedef!(Length => ScreenLength for Screen);

make_typedef!(Vector2D => OverworldVector for Overworld);
make_typedef!(Vector2D => ScreenVector for Screen);

pub trait OnScreen<Container> {
    fn on_screen(&self) -> Container;
}

macro_rules! make_on_screen_impl {
    ($underkate_name:ident => $mint_name:ident) => {
        impl<T: Copy> OnScreen<mint::$mint_name<T>> for $underkate_name<T> {
            fn on_screen(&self) -> mint::$mint_name<T> {
                let array: [T; 2] = (*self).into();
                array.into()
            }
        }
    };
}

make_on_screen_impl!(ScreenPoint => Point2);
