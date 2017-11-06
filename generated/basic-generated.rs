// This is what the call to gobject_gen! would generate in tests/basic.rs

pub mod CounterMod {
    extern crate glib_sys as glib_ffi ;
    extern crate gobject_sys as gobject_ffi ;
    extern crate glib ;
    extern crate libc ;
    use glib::{IsA, Value};
    use glib::object::Downcast;
    use glib::signal::connect;
    use glib::translate::*;
    use std::ptr;
    use std::mem;
    use std::mem::transmute;
    use super::*;
    glib_wrapper ! {
        pub struct Counter ( Object < imp :: Counter > ) ;
        match fn {
            get_type => || imp :: counter_get_type ( ) ,
        }
    }
    pub mod imp {
        use super::super::*;
        use super::glib;
        use super::glib_ffi;
        use super::gobject_ffi;
        use super::libc;
        use std::mem;
        use std::ptr;
        use glib::translate::*;
        #[repr(C)]
        pub struct Counter {
            pub parent: gobject_ffi::GObject,
        }
        #[repr(C)]
        pub struct CounterClass {
            pub parent_class: <glib::Object as glib::wrapper::Wrapper>::GlibClassType,
            pub add: Option<unsafe extern "C" fn(this: *mut Counter, x: u32) -> u32>,
            pub get: Option<unsafe extern "C" fn(this: *mut Counter) -> u32>,
        }
        #[repr(u32)]
        enum Properties {
            FIXMEDummy = 1,
        }
        #[repr(u32)]
        enum Signals {
            FIXMEDummy = 0,
        }
        struct CounterPrivate {
            f: Cell<u32>,
        }
        impl CounterPrivate {
            pub fn new() -> Self {
                CounterPrivate { f: Default::default() }
            }
        }
        struct CounterClassPrivate {
            parent_class: *const <glib::Object as glib::wrapper::Wrapper>::GlibClassType,
            properties: *const Vec<*const gobject_ffi::GParamSpec>,
            signals: *const Vec<u32>,
        }
        static mut PRIV: CounterClassPrivate = CounterClassPrivate {
            parent_class: 0 as *const _,
            properties: 0 as *const _,
            signals: 0 as *const _,
        };
        impl super::Counter {
            fn get_priv(&self) -> &CounterPrivate {
                unsafe {
                    let private = gobject_ffi::g_type_instance_get_private(
                        <Self as ToGlibPtr<*mut Counter>>::to_glib_none(self).0 as
                            *mut gobject_ffi::GTypeInstance,
                        counter_get_type(),
                    ) as *const Option<CounterPrivate>;
                    (&*private).as_ref().unwrap()
                }
            }
            fn add_impl(&self, x: u32) -> u32 {
                let private = self.get_priv();
                let v = private.f.get() + x;
                private.f.set(v);
                v
            }
            fn get_impl(&self) -> u32 {
                self.get_priv().f.get()
            }
        }
        impl Counter {
            fn get_class(&self) -> &CounterClass {
                unsafe {
                    let klass = (*(self as *const _ as *const gobject_ffi::GTypeInstance)).g_class;
                    &*(klass as *const CounterClass)
                }
            }
            unsafe extern "C" fn init(
                obj: *mut gobject_ffi::GTypeInstance,
                _klass: glib_ffi::gpointer,
            ) {
                let _guard = glib::CallbackGuard::new();
                let private = gobject_ffi::g_type_instance_get_private(obj, counter_get_type()) as
                    *mut Option<CounterPrivate>;
                ptr::write(private, Some(CounterPrivate::new()));
            }
            unsafe extern "C" fn finalize(obj: *mut gobject_ffi::GObject) {
                let _guard = glib::CallbackGuard::new();
                let private = gobject_ffi::g_type_instance_get_private(
                    obj as *mut gobject_ffi::GTypeInstance,
                    counter_get_type(),
                ) as *mut Option<CounterPrivate>;
                let _ = (*private).take();
                (*PRIV.parent_class).finalize.map(|f| f(obj));
            }
            unsafe extern "C" fn add_trampoline(this: *mut Counter, x: u32) -> u32 {
                let _guard = glib::CallbackGuard::new();
                let instance: &super::Counter = &from_glib_borrow(this);
                instance.add_impl(x)
            }
            unsafe extern "C" fn get_trampoline(this: *mut Counter) -> u32 {
                let _guard = glib::CallbackGuard::new();
                let instance: &super::Counter = &from_glib_borrow(this);
                instance.get_impl()
            }
        }
        impl CounterClass {
            unsafe extern "C" fn init(klass: glib_ffi::gpointer, _klass_data: glib_ffi::gpointer) {
                let _guard = glib::CallbackGuard::new();
                gobject_ffi::g_type_class_add_private(
                    klass,
                    mem::size_of::<Option<CounterPrivate>>(),
                );
                {
                    let gobject_class = &mut *(klass as *mut gobject_ffi::GObjectClass);
                    gobject_class.finalize = Some(Counter::finalize);
                }
                {
                    let klass = &mut *(klass as *mut CounterClass);
                    klass.add = Some(Counter::add_trampoline);
                    klass.get = Some(Counter::get_trampoline);
                }
                {}
                PRIV.parent_class = gobject_ffi::g_type_class_peek_parent(klass) as
                    *const <glib::Object as glib::wrapper::Wrapper>::GlibClassType;
            }
        }
        #[no_mangle]
        pub unsafe extern "C" fn counter_new() -> *mut Counter {
            let _guard = glib::CallbackGuard::new();
            let this = gobject_ffi::g_object_newv(counter_get_type(), 0, ptr::null_mut());
            this as *mut Counter
        }
        #[no_mangle]
        pub unsafe extern "C" fn counter_add(this: *mut Counter, x: u32) -> u32 {
            let _guard = glib::CallbackGuard::new();
            let klass = (*this).get_class();
            (klass.add.as_ref().unwrap())(this, x)
        }
        #[no_mangle]
        pub unsafe extern "C" fn counter_get(this: *mut Counter) -> u32 {
            let _guard = glib::CallbackGuard::new();
            let klass = (*this).get_class();
            (klass.get.as_ref().unwrap())(this)
        }
        #[no_mangle]
        pub unsafe extern "C" fn counter_get_type() -> glib_ffi::GType {
            let _guard = glib::CallbackGuard::new();
            use std::sync::{Once, ONCE_INIT};
            use std::u16;
            static mut TYPE: glib_ffi::GType = gobject_ffi::G_TYPE_INVALID;
            static ONCE: Once = ONCE_INIT;
            ONCE.call_once(|| {
                let class_size = mem::size_of::<CounterClass>();
                assert!(class_size <= u16::MAX as usize);
                let instance_size = mem::size_of::<Counter>();
                assert!(instance_size <= u16::MAX as usize);
                TYPE = gobject_ffi::g_type_register_static_simple(
                    <glib::Object as glib::StaticType>::static_type().to_glib(),
                    b"Counter\0" as *const u8 as *const i8,
                    class_size as u32,
                    Some(CounterClass::init),
                    instance_size as u32,
                    Some(Counter::init),
                    gobject_ffi::GTypeFlags::empty(),
                );
            });
            TYPE
        }
    }
    impl Counter {
        pub fn new() -> Counter {
            unsafe { from_glib_full(imp::counter_new()) }
        }
    }
    pub trait CounterExt {
        fn add(&self, x: u32) -> u32;
        fn get(&self) -> u32;
    }
    impl<O: IsA<Counter> + IsA<glib::object::Object>> CounterExt for O {
        fn add(&self, x: u32) -> u32 {
            unsafe { imp::counter_add(self.to_glib_none().0, x) }
        }
        fn get(&self) -> u32 {
            unsafe { imp::counter_get(self.to_glib_none().0) }
        }
    }
}
pub use CounterMod::*;
