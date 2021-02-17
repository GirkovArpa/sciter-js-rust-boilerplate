// #![windows_subsystem="windows"]
const NUMBER_OF_FUNCTIONS: usize = 3;
#[macro_use] extern crate sciter;
use sciter::{ Element, Value, FromValue, HELEMENT, types::{BOOL, VALUE} };
use std::thread;
#[derive(Default)]
pub struct Object {}
impl sciter::om::Passport for Object {
	fn get_passport(&self) -> &'static sciter::om::som_passport_t {
        use sciter::om::*;
        extern "C" fn on_sum(_thing: *mut som_asset_t, argc: u32, argv: *const VALUE, p_result: &mut VALUE) -> BOOL {
            let args = unsafe { sciter::Value::unpack_from(argv, argc) };
            let a: i32 = FromValue::from_value(&args[0]).unwrap();
            let b: i32 = FromValue::from_value(&args[1]).unwrap();
            let sum: i32 = a + b;
            let r: Value = Value::from(sum);
            r.pack_to(p_result);
			return true as BOOL;
        }
        extern "C" fn on_sum_async(_thing: *mut som_asset_t, argc: u32, argv: *const VALUE, p_result: &mut VALUE) -> BOOL {
            let args = unsafe { sciter::Value::unpack_from(argv, argc) };
            let a: i32 = FromValue::from_value(&args[0]).unwrap();
            let b: i32 = FromValue::from_value(&args[1]).unwrap();
            let callback: Value = FromValue::from_value(&args[2]).unwrap();
            let sum: i32 = a + b;
            let r: Value = Value::from(true);
            r.pack_to(p_result);
            thread::spawn(move || {
                callback.call(None, &make_args!(sum), None).unwrap();
            });
			return true as BOOL;
        }
        extern "C" fn on_capitalize(_thing: *mut som_asset_t, argc: u32, argv: *const VALUE, p_result: &mut VALUE) -> BOOL {
            let args = unsafe { sciter::Value::unpack_from(argv, argc) };
            let string: String = FromValue::from_value(&args[0]).unwrap();
            let r = Value::from(string.to_string().as_str().to_uppercase());
            r.pack_to(p_result);
            return true as BOOL;
        }
		type ObjectMethods = [som_method_def_t; NUMBER_OF_FUNCTIONS];
		let mut methods = Box::new(ObjectMethods::default());
        let mut method = &mut methods[0];
		method.name = atom("sum");
		method.func = Some(on_sum);
        method.params = 2;
        let mut method = &mut methods[1];
		method.name = atom("sum_async");
		method.func = Some(on_sum_async);
        method.params = 3;
        let mut method = &mut methods[2];
		method.name = atom("capitalize");
		method.func = Some(on_capitalize);
        method.params = 1;
		let mut pst = Box::new(som_passport_t::default());
		pst.name = atom("rust");
		pst.n_methods = NUMBER_OF_FUNCTIONS;
        pst.methods = Box::into_raw(methods) as *const _;
		Box::leak(pst)
	}
}
#[derive(Debug)]
struct Handler { asset: sciter::om::IAssetRef<Object> }
impl sciter::EventHandler for Handler {
    fn attached(&mut self, root: HELEMENT) {
        println!("attached"); // prints
		Element::from(root).call_function("set_title", &make_args!("quick maths!")); // does nothing
	}
    fn get_asset(&mut self) -> Option<&sciter::om::som_asset_t> {
		Some(self.asset.as_ref())
    }
}
fn main() {
    sciter::set_options(sciter::RuntimeOptions::DebugMode(false)).unwrap();
    sciter::set_options(sciter::RuntimeOptions::ScriptFeatures(
        sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SYSINFO  as u8 |
        sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_FILE_IO  as u8 |
        sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_EVAL     as u8 |
        sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SYSINFO  as u8 
    )).unwrap();
	let object = Object::default();
	let object = sciter::om::IAsset::new(object);
    let object = sciter::om::IAssetRef::from(object);
    let mut frame = sciter::Window::new();
	let handler = Handler { asset: object };
	frame.event_handler(handler);
    let archived = include_bytes!("../target/assets.rc");
    frame.archive_handler(archived).unwrap();
    frame.load_file("this://app/main.htm");
    frame.run_app();
}
