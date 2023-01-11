//! Using memory reporting

use mlua::Lua;

fn main() {
    let script = r#"
        local a = "test"
        local table = {}
        table["key"] = "a string long enough that it won't be cached"
        fun = function()
            local b = a
        end
    "#;

    let lua = Lua::new();
    let report = lua.mem_report();
    println!("{:#?}", report);

    lua.load(script).exec().unwrap();
    let report = lua.mem_report();
    // Should show an increase in some entries due to the script running
    println!("{:#?}", report);
}

