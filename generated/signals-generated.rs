// This is what the call to gobject_gen! would generate in tests/signals.rs

pub mod SignalerMod {
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
        pub struct Signaler ( Object < imp :: Signaler > ) ;
        match fn {
            get_type => || imp :: signaler_get_type ( ) ,
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
        pub struct Signaler {
            pub parent: gobject_ffi::GObject,
        }
        #[repr(C)]
        pub struct SignalerClass {
            pub parent_class: <glib::Object as glib::wrapper::Wrapper>::GlibClassType,
            pub set_value: Option<unsafe extern "C" fn(this: *mut Signaler, v: u32)>,
            pub get_value: Option<unsafe extern "C" fn(this: *mut Signaler) -> u32>,
        }
        #[repr(u32)]
        enum Properties {
            FIXMEDummy = 1,
        }
        #[repr(u32)]
        enum Signals {
            FIXMEDummy = 0,
        }
        struct SignalerPrivate {
            val: Cell<u32>,
        }
        impl SignalerPrivate {
            pub fn new() -> Self {
                SignalerPrivate { val: Default::default() }
            }
        }
        struct SignalerClassPrivate {
            parent_class: *const <glib::Object as glib::wrapper::Wrapper>::GlibClassType,
            properties: *const Vec<*const gobject_ffi::GParamSpec>,
            signals: *const Vec<u32>,
        }
        static mut PRIV: SignalerClassPrivate = SignalerClassPrivate {
            parent_class: 0 as *const _,
            properties: 0 as *const _,
            signals: 0 as *const _,
        };
        impl super::Signaler {
            fn get_priv(&self) -> &SignalerPrivate {
                unsafe {
                    let private = gobject_ffi::g_type_instance_get_private(
                        <Self as ToGlibPtr<*mut Signaler>>::to_glib_none(self).0 as
                            *mut gobject_ffi::GTypeInstance,
                        signaler_get_type(),
                    ) as *const Option<SignalerPrivate>;
                    (&*private).as_ref().unwrap()
                }
            }
            fn set_value_impl(&self, v: u32) {
                let private = self.get_priv();
                private.val.set(v);
            }
            fn get_value_impl(&self) -> u32 {
                let private = self.get_priv();
                private.val.get()
            }
        }
        impl Signaler {
            fn get_class(&self) -> &SignalerClass {
                unsafe {
                    let klass = (*(self as *const _ as *const gobject_ffi::GTypeInstance)).g_class;
                    &*(klass as *const SignalerClass)
                }
            }
            unsafe extern "C" fn init(
                obj: *mut gobject_ffi::GTypeInstance,
                _klass: glib_ffi::gpointer,
            ) {
                let _guard = glib::CallbackGuard::new();
                let private =
                    gobject_ffi::g_type_instance_get_private(obj, signaler_get_type()) as
                        *mut Option<SignalerPrivate>;
                ptr::write(private, Some(SignalerPrivate::new()));
            }
            unsafe extern "C" fn finalize(obj: *mut gobject_ffi::GObject) {
                let _guard = glib::CallbackGuard::new();
                let private = gobject_ffi::g_type_instance_get_private(
                    obj as *mut gobject_ffi::GTypeInstance,
                    signaler_get_type(),
                ) as *mut Option<SignalerPrivate>;
                let _ = (*private).take();
                (*PRIV.parent_class).finalize.map(|f| f(obj));
            }
            unsafe extern "C" fn set_value_trampoline(this: *mut Signaler, v: u32) {
                let _guard = glib::CallbackGuard::new();
                let instance: &super::Signaler = &from_glib_borrow(this);
                instance.set_value_impl(v)
            }
            unsafe extern "C" fn get_value_trampoline(this: *mut Signaler) -> u32 {
                let _guard = glib::CallbackGuard::new();
                let instance: &super::Signaler = &from_glib_borrow(this);
                instance.get_value_impl()
            }
        }
        impl SignalerClass {
            unsafe extern "C" fn init(klass: glib_ffi::gpointer, _klass_data: glib_ffi::gpointer) {
                let _guard = glib::CallbackGuard::new();
                gobject_ffi::g_type_class_add_private(
                    klass,
                    mem::size_of::<Option<SignalerPrivate>>(),
                );
                {
                    let gobject_class = &mut *(klass as *mut gobject_ffi::GObjectClass);
                    gobject_class.finalize = Some(Signaler::finalize);
                }
                {
                    let klass = &mut *(klass as *mut SignalerClass);
                    klass.set_value = Some(Signaler::set_value_trampoline);
                    klass.get_value = Some(Signaler::get_value_trampoline);
                }
                {}
                PRIV.parent_class = gobject_ffi::g_type_class_peek_parent(klass) as
                    *const <glib::Object as glib::wrapper::Wrapper>::GlibClassType;
            }
        }
        #[no_mangle]
        pub unsafe extern "C" fn signaler_new() -> *mut Signaler {
            let _guard = glib::CallbackGuard::new();
            let this = gobject_ffi::g_object_newv(signaler_get_type(), 0, ptr::null_mut());
            this as *mut Signaler
        }
        #[no_mangle]
        pub unsafe extern "C" fn signaler_set_value(this: *mut Signaler, v: u32) {
            let _guard = glib::CallbackGuard::new();
            let klass = (*this).get_class();
            (klass.set_value.as_ref().unwrap())(this, v)
        }
        #[no_mangle]
        pub unsafe extern "C" fn signaler_get_value(this: *mut Signaler) -> u32 {
            let _guard = glib::CallbackGuard::new();
            let klass = (*this).get_class();
            (klass.get_value.as_ref().unwrap())(this)
        }
        #[no_mangle]
        pub unsafe extern "C" fn signaler_get_type() -> glib_ffi::GType {
            let _guard = glib::CallbackGuard::new();
            use std::sync::{Once, ONCE_INIT};
            use std::u16;
            static mut TYPE: glib_ffi::GType = gobject_ffi::G_TYPE_INVALID;
            static ONCE: Once = ONCE_INIT;
            ONCE.call_once(|| {
                let class_size = mem::size_of::<SignalerClass>();
                assert!(class_size <= u16::MAX as usize);
                let instance_size = mem::size_of::<Signaler>();
                assert!(instance_size <= u16::MAX as usize);
                TYPE = gobject_ffi::g_type_register_static_simple(
                    <glib::Object as glib::StaticType>::static_type().to_glib(),
                    b"Signaler\0" as *const u8 as *const i8,
                    class_size as u32,
                    Some(SignalerClass::init),
                    instance_size as u32,
                    Some(Signaler::init),
                    gobject_ffi::GTypeFlags::empty(),
                );
            });
            TYPE
        }
    }
    impl Signaler {
        pub fn new() -> Signaler {
            unsafe { from_glib_full(imp::signaler_new()) }
        }
    }
    pub trait SignalerExt {
        fn set_value(&self, v: u32);
        fn get_value(&self) -> u32;
    }
    impl<O: IsA<Signaler> + IsA<glib::object::Object>> SignalerExt for O {
        fn set_value(&self, v: u32) {
            unsafe { imp::signaler_set_value(self.to_glib_none().0, v) }
        }
        fn get_value(&self) -> u32 {
            unsafe { imp::signaler_get_value(self.to_glib_none().0) }
        }
    }
}
pub use SignalerMod::*;
