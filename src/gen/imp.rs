use super::*;

impl<'ast> ClassContext<'ast> {
    pub fn slots(&self) -> Vec<Tokens> {
        // ABI: we are generating the imp::FooClass with the parent_class, and the slots to signals/methods.
        // This defines the C ABI for the class structure.
        //
        // FIXME: we should check that the extern "C" signatures only have types representable by C.

        /*
        self.members_with_slots()
            .map(|member| {
                let (slot_name, slot_fn_ty) = match *member {
                    Member::Method(ref method) => (method.name,
                                                   SlotTy {
                                                       class_name: self.InstanceName,
                                                       sig: &method.fn_def.sig
                                                   }),

                    Member::Signal(ref signal) => (signal.name,
                                                   SlotTy {
                                                       class_name: self.InstanceName,
                                                       sig: &signal.sig
                                                   }),

                    _ => unreachable! ()
                };

                quote! {
                    pub #slot_name: Option<unsafe extern "C" fn#slot_fn_ty>,
                }
            })
            .collect()
        */
        Vec::new()
    }

    pub fn private_init_fn_body(&self) -> Tokens {
        // If the user had a "private_init()" method, we want to use it as an initializer
        // for the private struct.
        //
        // Otherwise, just initialize all of the struct's fields to Default::default().

        let private_init_item =
            self.class.items
            .iter()
            .filter_map(|i| match *i {
                ClassItem::PrivateInit(ref f) => Some(f),
                _ => None,
            })
            .next();

        if let Some(i) = private_init_item {
            // FIXME: check that i.inputs is empty
            // FIXME: check that i.output is the same as PrivateStruct
            let block = &i.block;
            quote! { #block }
        } else {
            panic!("Class must have a private_init() item");
            /*
            let PrivateName = self.private_struct.name();
            // FIXME: self.private_struct.fields is no longer Vec<VarTy>; it is now syn::VariantData.
            let private_struct_field_names =
                self.private_struct.fields
                                   .iter()
                                   .map(|f| f.ident.as_ref().unwrap());
            quote! {
                {
                    #PrivateName {
                        #(#private_struct_field_names: Default::default()),*
                    }
                }
            }
            */
        }
    }

    pub fn imp_slot_default_handlers(&self) -> Vec<Tokens> {
        /*
        self.members_with_slots()
            .map(|member| {
                let (slot_name, slot_impl_ty, code) = match *member {
                    Member::Method(ref method) => (&method.name,
                                                   SlotImplTy {
                                                       sig:  &method.fn_def.sig
                                                   },
                                                   &method.fn_def.code),

                    Member::Signal(ref signal) => (&signal.name,
                                                   SlotImplTy {
                                                       sig:  &signal.sig
                                                   },
                                                   signal.code.as_ref().unwrap()), // FIXME: signals with no default handler?
                    _ => unreachable!()
                };

                let slot_impl_name = Self::slot_impl_name(slot_name);

                quote! {
                    fn #slot_impl_name#slot_impl_ty #code
                }
            })
            .collect()
         */
        Vec::new()
    }

    pub fn instance_method_trampolines(&self) -> Vec<Tokens> {
        /*
        let callback_guard = self.glib_callback_guard();
        let InstanceName = self.InstanceName;
        self.methods()
            .map(|method| {
                let trampoline_name = Self::slot_trampoline_name(&method.name);
                let method_impl_name = Self::slot_impl_name(&method.name);

                let slot_ty = SlotTy {
                    class_name: self.InstanceName,
                    sig: &method.fn_def.sig
                };

                let arg_names = method.fn_def.sig.arg_names();

                quote! {
                    unsafe extern "C" fn #trampoline_name#slot_ty {
                        #callback_guard

                        let instance: &super::#InstanceName = &from_glib_borrow(this);

                        // FIXME: do we need to from_glib_*() each argument?
                        // FIXME: do we need to .to_glib() the return value?
                        instance.#method_impl_name(#arg_names)
                    }
                }
            })
            .collect()
         */
        Vec::new()
    }

    pub fn instance_signal_trampolines(&self) -> Vec<Tokens> {
        // FIXME
        Vec::new()
    }

    pub fn instance_method_impls(&self) -> Vec<Tokens> {
        // FIXME
        Vec::new()
    }

    pub fn instance_default_signal_handlers(&self) -> Vec<Tokens> {
        // FIXME
        Vec::new()
    }

    pub fn imp_extern_methods(&self) -> Vec<Tokens> {
        /*
        let InstanceName = self.InstanceName;
        let callback_guard = self.glib_callback_guard();

        self.methods()
            .map(|method| {
                let arg_decls   = method.fn_def.sig.arg_decls();
                let arg_names   = method.fn_def.sig.arg_names();
                let return_ty   = method.fn_def.sig.return_ty();
                let method_name = method.name;
                let ffi_name    = self.method_ffi_name(method);

                quote! {
                    #[no_mangle]
                    pub unsafe extern "C" fn #ffi_name(this: *mut #InstanceName, #arg_decls) #return_ty {
                        #callback_guard

                        let klass = (*this).get_class();
                        // We unwrap() because klass.method_name is always set to a method_trampoline
                        (klass.#method_name.as_ref().unwrap())(this, #arg_names)
                    }
                }
            })
            .collect()
         */
        Vec::new()
    }

/*
    pub fn members_with_slots(&self) -> impl Iterator<Item = &'ast Member> {
        self.class
            .members
            .iter()
            .filter_map(|member| match *member {
                Member::Method(_) => Some(member),
                Member::Signal(_) => Some(member),
                _ => None,
            })
    }
*/

/*
    fn slot_trampoline_name(slot_name: &Ident) -> Ident {
        Ident::from(&format!("{}_trampoline", slot_name.as_ref()))
    }
*/

/*
    fn slot_impl_name(slot_name: &Ident) -> Ident {
        Ident::from(&format!("{}_impl", slot_name.as_ref()))
    }
*/

    pub fn slot_assignments(&self) -> Vec<Tokens> {
        /*
        let InstanceName = self.InstanceName;

        self.members_with_slots()
            .map(|member| {
                let slot_name = match *member {
                    Member::Method(ref method) => method.name,
                    Member::Signal(ref signal) => signal.name,
                    _ => unreachable!()
                };

                let trampoline_name = Self::slot_trampoline_name(&slot_name);

                quote! {
                    klass.#slot_name = Some(#InstanceName::#trampoline_name);
                }
            })
            .collect()
         */
        Vec::new()
    }

    pub fn imp_new_fn_name(&self) -> Ident {
        self.exported_fn_name("new")
    }
}
