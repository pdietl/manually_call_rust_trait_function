use std::mem;

trait Showable {
    fn show_me(&self);
}

impl Showable for &[i32] {
    fn show_me(&self) {
        println!("showing me: {:?}", self);
    }
}

fn show_it(x: &dyn Showable) {
    dbg!(
        unsafe {
            mem::transmute::<_, (*const usize, *const usize)>(x)
        }
    );

    unsafe {
        let (addr_of_slice, vtable) = mem::transmute::<_, (*const &[i32], usize)>(x);
        /* first three are:
               callee::get_fn(cx, monomorphize::resolve_drop_in_place(cx.tcx, ty)),
               C_usize(cx, size.bytes()),
               C_usize(cx, align.abi())
           then comes our trait's function
        */
        let show_me = &*((vtable as *const fn(&&[i32])).offset(3));

        let slice = *addr_of_slice;

        for i in 0..4 {
            println!("0x{:x}", *(vtable as *const usize).offset(i));
        }

        dbg!(slice);

        show_me(&slice);
    };
}

fn main() {
    let array = [0, 1, 1, 2, 3, 5];
    let slice = &array[..];

    slice.show_me();

    show_it(&slice);
}
