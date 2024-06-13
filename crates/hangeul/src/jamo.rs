use enum_map::Enum;
use serde::{Deserialize, Serialize};

pub type JamoIndex = u32;

pub trait Jamo: Sized {
    fn length() -> usize;
    fn from_index(index: JamoIndex) -> Option<Self>;
    fn index(self) -> JamoIndex;

    fn next(self) -> Option<Self> {
        Self::from_index(self.index() + 1)
    }
}

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

macro_rules! define_jamo{
    ($name: ident : $($element: ident),* $(,)?) => {
	#[derive(Debug, Copy, Clone, Serialize, Deserialize, Enum)]
	pub enum $name{
	    $($element),*
	}


	impl $name{
	    pub const LENGTH: usize = count!($($element)*);
	}

	impl Jamo for $name{
	    fn length() -> usize{
		Self::LENGTH
	    }

	    fn from_index(index: JamoIndex) -> Option<Self>{
		match index{
		    $( _ if index == Self::$element.index() => Some($name::$element), )*
		    _ => None
		}
	    }

	    fn index(self) -> JamoIndex{
		self as JamoIndex
	    }
	}
    }
}

define_jamo! {Choseong:
    ㄱ, ㄲ,
    ㄴ,
    ㄷ, ㄸ,
    ㄹ,
    ㅁ,
    ㅂ, ㅃ,
    ㅅ, ㅆ,
    ㅇ,
    ㅈ, ㅉ,
    ㅊ,
    ㅋ,
    ㅌ,
    ㅍ,
    ㅎ,
}

define_jamo! {Jungseong:
    ㅏ,
    ㅐ,
    ㅑ,
    ㅒ,
    ㅓ,
    ㅔ,
    ㅕ,
    ㅖ,
    ㅗ,
    ㅘ,
    ㅙ,
    ㅚ,
    ㅛ,
    ㅜ,
    ㅝ,
    ㅞ,
    ㅟ,
    ㅠ,
    ㅡ,
    ㅢ,
    ㅣ,
}

define_jamo! {Jongseong:
    None,
    ㄱ,
    ㄲ,
    ㄳ,
    ㄴ,
    ㄵ,
    ㄶ,
    ㄷ,
    ㄹ,
    ㄺ,
    ㄻ,
    ㄼ,
    ㄽ,
    ㄾ,
    ㄿ,
    ㅀ,
    ㅁ,
    ㅂ,
    ㅄ,
    ㅅ,
    ㅆ,
    ㅇ,
    ㅈ,
    ㅊ,
    ㅋ,
    ㅌ,
    ㅍ,
    ㅎ,
}
