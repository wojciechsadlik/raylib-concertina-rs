use std::collections::HashMap;
use std::sync::LazyLock;
use raylib::consts::KeyboardKey;


pub static KEYBOARD_MAP: LazyLock<HashMap<KeyboardKey, String>> = LazyLock::new(|| {
    HashMap::from([
        (KeyboardKey::KEY_F, "l7".to_string()),
        (KeyboardKey::KEY_G, "l6".to_string()),
    ])
});


pub static BTN_MAP: LazyLock<HashMap<String, i32>> = LazyLock::new(|| {
    HashMap::from([
        ("l7ps".to_string(), 64_i32),
        ("l7pl".to_string(), 65_i32),
        ("l6ps".to_string(), 67_i32),
        ("l6pl".to_string(), 69_i32),
    ])
});
