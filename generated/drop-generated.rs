// This is what the call to gobject_gen! would generate in tests/drop.rs

pub mod DummyMod {
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
        pub struct Dummy ( Object < imp :: Dummy > );
        match fn {
            get_type => || imp :: dummy_get_type ( ) ,
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
        pub struct Dummy {
            pub parent: gobject_ffi::GObject,
        }
        #[repr(C)]
        pub struct DummyClass {
            pub parent_class: <glib::Object as glib::wrapper::Wrapper>::GlibClassType,
            pub set_dc: Option<unsafe extern "C" fn(this: *mut Dummy, dc: DropCounter)>,
        }
        #[repr(u32)]
        enum Properties {
            FIXMEDummy = 1,
        }
        #[repr(u32)]
        enum Signals {
            FIXMEDummy = 0,
        }
        struct DummyPrivate {
            dc: RefCell<DropCounter>,
        }
        impl DummyPrivate {
            pub fn new() -> Self {
                DummyPrivate { dc: RefCell::new(DropCounter::new()) }
            }
        }
        struct DummyClassPrivate {
            parent_class: *const <glib::Object as glib::wrapper::Wrapper>::GlibClassType,
            properties: *const Vec<*const gobject_ffi::GParamSpec>,
            signals: *const Vec<u32>,
        }
        static mut PRIV: DummyClassPrivate = DummyClassPrivate {
            parent_class: 0 as *const _,
            properties: 0 as *const _,
            signals: 0 as *const _,
        };
        impl super::Dummy {
            fn get_priv(&self) -> &DummyPrivate {
                unsafe {
                    let private = gobject_ffi::g_type_instance_get_private(
                        <Self as ToGlibPtr<*mut Dummy>>::to_glib_none(self).0 as
                            *mut gobject_ffi::GTypeInstance,
                        dummy_get_type(),
                    ) as *const Option<DummyPrivate>;
                    (&*private).as_ref().unwrap()
                }
            }
            fn set_dc_impl(&self, dc: DropCounter) {
                let mut self_dc = self.get_priv().dc.borrow_mut();
                *self_dc = dc;
            }
        }
        impl Dummy {
            fn get_class(&self) -> &DummyClass {
                unsafe {
                    let klass = (*(self as *const _ as *const gobject_ffi::GTypeInstance)).g_class;
                    &*(klass as *const DummyClass)
                }
            }
            unsafe extern "C" fn init(
                obj: *mut gobject_ffi::GTypeInstance,
                _klass: glib_ffi::gpointer,
            ) {
                let _guard = glib::CallbackGuard::new();
                let private = gobject_ffi::g_type_instance_get_private(obj, dummy_get_type()) as
                    *mut Option<DummyPrivate>;
                ptr::write(private, Some(DummyPrivate::new()));
            }
            unsafe extern "C" fn finalize(obj: *mut gobject_ffi::GObject) {
                let _guard = glib::CallbackGuard::new();
                let private = gobject_ffi::g_type_instance_get_private(
                    obj as *mut gobject_ffi::GTypeInstance,
                    dummy_get_type(),
                ) as *mut Option<DummyPrivate>;
                let _ = (*private).take();
                (*PRIV.parent_class).finalize.map(|f| f(obj));
            }
            unsafe extern "C" fn set_dc_trampoline(this: *mut Dummy, dc: DropCounter) {
                let _guard = glib::CallbackGuard::new();
                let instance: &super::Dummy = &from_glib_borrow(this);
                instance.set_dc_impl(dc)
            }
        }
        impl DummyClass {
            unsafe extern "C" fn init(klass: glib_ffi::gpointer, _klass_data: glib_ffi::gpointer) {
                let _guard = glib::CallbackGuard::new();
                gobject_ffi::g_type_class_add_private(
                    klass,
                    mem::size_of::<Option<DummyPrivate>>(),
                );
                {
                    let gobject_class = &mut *(klass as *mut gobject_ffi::GObjectClass);
                    gobject_class.finalize = Some(Dummy::finalize);
                }
                {
                    let klass = &mut *(klass as *mut DummyClass);
                    klass.set_dc = Some(Dummy::set_dc_trampoline);
                }
                {}
                PRIV.parent_class = gobject_ffi::g_type_class_peek_parent(klass) as
                    *const <glib::Object as glib::wrapper::Wrapper>::GlibClassType;
            }
        }
        #[no_mangle]
        pub unsafe extern "C" fn dummy_new() -> *mut Dummy {
            let _guard = glib::CallbackGuard::new();
            let this = gobject_ffi::g_object_newv(dummy_get_type(), 0, ptr::null_mut());
            this as *mut Dummy
        }
        #[no_mangle]
        pub unsafe extern "C" fn dummy_set_dc(this: *mut Dummy, dc: DropCounter) {
            let _guard = glib::CallbackGuard::new();
            let klass = (*this).get_class();
            (klass.set_dc.as_ref().unwrap())(this, dc)
        }
        #[no_mangle]
        pub unsafe extern "C" fn dummy_get_type() -> glib_ffi::GType {
            let _guard = glib::CallbackGuard::new();
            use std::sync::{Once, ONCE_INIT};
            use std::u16;
            static mut TYPE: glib_ffi::GType = gobject_ffi::G_TYPE_INVALID;
            static ONCE: Once = ONCE_INIT;
            ONCE.call_once(|| {
                let class_size = mem::size_of::<DummyClass>();
                assert!(class_size <= u16::MAX as usize);
                let instance_size = mem::size_of::<Dummy>();
                assert!(instance_size <= u16::MAX as usize);
                TYPE = gobject_ffi::g_type_register_static_simple(
                    <glib::Object as glib::StaticType>::static_type().to_glib(),
                    b"Dummy\0" as *const u8 as *const i8,
                    class_size as u32,
                    Some(DummyClass::init),
                    instance_size as u32,
                    Some(Dummy::init),
                    gobject_ffi::GTypeFlags::empty(),
                );
            });
            TYPE
        }
    }
    impl Dummy {
        pub fn new() -> Dummy {
            unsafe { from_glib_full(imp::dummy_new()) }
        }
    }
    pub trait DummyExt {
        fn set_dc(&self, dc: DropCounter);
    }
    impl<O: IsA<Dummy> + IsA<glib::object::Object>> DummyExt for O {
        fn set_dc(&self, dc: DropCounter) {
            unsafe { imp::dummy_set_dc(self.to_glib_none().0, dc) }
        }
    }
}
pub use DummyMod::*;
