#[cfg(all(feature = "lua51-wasi", target_arch = "wasm32"))]
mod wasi_tests {
    use mlua::prelude::*;

    #[test]
    fn test_basic_globals() {
        let lua = Lua::new();

        // Test that basic globals are available and are the correct types
        let globals = lua.globals();

        // _G should be a table that references itself
        let global_g: LuaTable = globals.get("_G").expect("_G should exist");
        assert!(global_g.contains_key("_G").expect("_G should contain itself"));

        // Basic functions should exist
        let print_func: LuaFunction = globals.get("print").expect("print should exist");
        let type_func: LuaFunction = globals.get("type").expect("type should exist");
        let pcall_func: LuaFunction = globals.get("pcall").expect("pcall should exist");

        // Test that type function works
        let result: String = type_func.call("hello").expect("type('hello') should work");
        assert_eq!(result, "string");

        let result: String = type_func.call(42).expect("type(42) should work");
        assert_eq!(result, "number");

        let result: String = type_func
            .call(print_func.clone())
            .expect("type(print) should work");
        assert_eq!(result, "function");
    }

    #[test]
    fn test_standard_libraries() {
        let lua = Lua::new();
        let globals = lua.globals();

        // Test math library
        let math_table: LuaTable = globals.get("math").expect("math library should exist");
        let math_floor: LuaFunction = math_table.get("floor").expect("math.floor should exist");
        let result: i32 = math_floor.call(3.7).expect("math.floor(3.7) should work");
        assert_eq!(result, 3);

        // Test string library
        let string_table: LuaTable = globals.get("string").expect("string library should exist");
        let string_upper: LuaFunction = string_table.get("upper").expect("string.upper should exist");
        let result: String = string_upper
            .call("hello")
            .expect("string.upper('hello') should work");
        assert_eq!(result, "HELLO");

        // Test table library
        let table_table: LuaTable = globals.get("table").expect("table library should exist");
        let table_insert: LuaFunction = table_table.get("insert").expect("table.insert should exist");

        // Test os library (basic functionality)
        let os_table: LuaTable = globals.get("os").expect("os library should exist");
        let os_time: LuaFunction = os_table.get("time").expect("os.time should exist");
        let _time_result: f64 = os_time.call(()).expect("os.time() should work");
    }

    #[test]
    fn test_lua_code_execution() {
        let lua = Lua::new();

        // Test basic arithmetic
        let result: i32 = lua.load("return 2 + 3").eval().expect("2 + 3 should work");
        assert_eq!(result, 5);

        // Test string operations
        let result: String = lua
            .load(r#"return "Hello, " .. "World!""#)
            .eval()
            .expect("String concatenation should work");
        assert_eq!(result, "Hello, World!");

        // Test table operations
        let result: i32 = lua
            .load(
                r#"
            local t = {1, 2, 3}
            table.insert(t, 4)
            return #t
        "#,
            )
            .eval()
            .expect("Table operations should work");
        assert_eq!(result, 4);

        // Test function definition and call
        let result: i32 = lua
            .load(
                r#"
            local function add(a, b)
                return a + b
            end
            return add(10, 20)
        "#,
            )
            .eval()
            .expect("Function definition and call should work");
        assert_eq!(result, 30);
    }

    #[test]
    fn test_type_checking() {
        let lua = Lua::new();

        // Verify that globals are actually tables, not strings
        let code = r#"
            local function check_type(name, expected_type)
                local actual_type = type(_G[name])
                if actual_type ~= expected_type then
                    error(string.format("Expected %s to be %s, but got %s", name, expected_type, actual_type))
                end
            end

            check_type("math", "table")
            check_type("string", "table")
            check_type("table", "table")
            check_type("os", "table")
            check_type("_G", "table")
            check_type("print", "function")
            check_type("type", "function")
            check_type("pcall", "function")

            return "All type checks passed!"
        "#;

        let result: String = lua.load(code).eval().expect("Type checks should pass");
        assert_eq!(result, "All type checks passed!");
    }

    #[test]
    fn test_global_g_self_reference() {
        let lua = Lua::new();

        // Test that _G references itself correctly
        let code = r#"
            -- _G should reference itself
            if _G._G ~= _G then
                error("_G does not reference itself correctly")
            end

            -- _G should be accessible through itself
            if _G._G._G._G.print ~= print then
                error("_G self-reference chain is broken")
            end

            return "Self-reference test passed!"
        "#;

        let result: String = lua.load(code).eval().expect("_G self-reference should work");
        assert_eq!(result, "Self-reference test passed!");
    }
}

#[cfg(not(all(feature = "lua51-wasi", target_arch = "wasm32")))]
fn main() {
    println!("WASI tests can only run on wasm32-wasip1 target with lua51-wasi feature");
}
