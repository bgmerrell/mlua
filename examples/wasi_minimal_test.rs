#[cfg(all(feature = "lua51-wasi", target_arch = "wasm32"))]
fn main() {
    use mlua::prelude::*;

    println!("Starting WASI Lua integration test...");

    // Create Lua instance
    let lua = Lua::new();
    println!("✓ Lua instance created");

    // Test basic globals exist and have correct types
    let globals = lua.globals();

    // Test _G is a table that references itself
    let global_g: LuaTable = globals.get("_G").expect("_G should exist");
    assert!(global_g.contains_key("_G").expect("_G should contain itself"));
    println!("✓ _G self-reference works");

    // Test basic functions exist
    let _print_func: LuaFunction = globals.get("print").expect("print should exist");
    let type_func: LuaFunction = globals.get("type").expect("type should exist");
    let _pcall_func: LuaFunction = globals.get("pcall").expect("pcall should exist");
    println!("✓ Basic functions exist");

    // Test type function works
    let result: String = type_func.call("hello").expect("type('hello') should work");
    assert_eq!(result, "string");
    println!("✓ type() function works");

    // Test that standard libraries are tables, not strings
    let math_table: LuaTable = globals.get("math").expect("math should be a table");
    let string_table: LuaTable = globals.get("string").expect("string should be a table");
    let _table_table: LuaTable = globals.get("table").expect("table should be a table");
    let _os_table: LuaTable = globals.get("os").expect("os should be a table");
    println!("✓ Standard libraries are tables");

    // Test math.floor function
    let math_floor: LuaFunction = math_table.get("floor").expect("math.floor should exist");
    let result: i32 = math_floor.call(3.7).expect("math.floor(3.7) should work");
    assert_eq!(result, 3);
    println!("✓ math.floor() works");

    // Test string.upper function
    let string_upper: LuaFunction = string_table.get("upper").expect("string.upper should exist");
    let result: String = string_upper
        .call("hello")
        .expect("string.upper('hello') should work");
    assert_eq!(result, "HELLO");
    println!("✓ string.upper() works");

    // Test Lua code execution
    let result: i32 = lua.load("return 2 + 3").eval().expect("2 + 3 should work");
    assert_eq!(result, 5);
    println!("✓ Basic arithmetic works");

    // Test string concatenation
    let result: String = lua
        .load(r#"return "Hello, " .. "World!""#)
        .eval()
        .expect("String concatenation should work");
    assert_eq!(result, "Hello, World!");
    println!("✓ String concatenation works");

    // Test that globals are actually tables with comprehensive type checking
    let code = r#"
        local function check_type(name, expected_type)
            local value = _G[name]
            local actual_type = type(value)
            if actual_type ~= expected_type then
                error(string.format("Expected %s to be %s, but got %s (value: %s)",
                    name, expected_type, actual_type, tostring(value)))
            end
            return true
        end

        check_type("math", "table")
        check_type("string", "table")
        check_type("table", "table")
        check_type("os", "table")
        check_type("_G", "table")
        check_type("print", "function")
        check_type("type", "function")
        check_type("pcall", "function")

        return "All types correct!"
    "#;

    let result: String = lua.load(code).eval().expect("Type checks should pass");
    assert_eq!(result, "All types correct!");
    println!("✓ All library types are correct");

    // Test _G self-reference chain
    let code = r#"
        if _G._G ~= _G then
            error("_G does not reference itself correctly")
        end

        if _G._G._G._G.print ~= print then
            error("_G self-reference chain is broken")
        end

        return "_G self-reference works!"
    "#;

    let result: String = lua.load(code).eval().expect("_G self-reference should work");
    assert_eq!(result, "_G self-reference works!");
    println!("✓ _G self-reference chain works");

    println!("\n🎉 All WASI Lua integration tests passed!");
    println!("The fix successfully resolves the library initialization issues.");
}

#[cfg(not(all(feature = "lua51-wasi", target_arch = "wasm32")))]
fn main() {
    println!("This example can only run on wasm32-wasip1 target with lua51-wasi feature enabled.");
    println!("Build with: WASI_SDK=/path/to/wasi-sdk cargo build --example wasi_minimal_test --features \"lua51-wasi,vendored\" --target wasm32-wasip1");
}
