extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type __int64_t = i64;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picosplay_node_t {
    pub parent: *mut st_picosplay_node_t,
    pub left: *mut st_picosplay_node_t,
    pub right: *mut st_picosplay_node_t,
}
pub type picosplay_node_t = st_picosplay_node_t;
pub type picosplay_comparator =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> int64_t>;
pub type picosplay_create =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut picosplay_node_t>;
pub type picosplay_delete_node =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut picosplay_node_t) -> ()>;
pub type picosplay_node_value =
    Option<unsafe extern "C" fn(*mut picosplay_node_t) -> *mut ::core::ffi::c_void>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picosplay_tree_t {
    pub root: *mut picosplay_node_t,
    pub comp: picosplay_comparator,
    pub create: picosplay_create,
    pub delete_node: picosplay_delete_node,
    pub node_value: picosplay_node_value,
    pub size: ::core::ffi::c_int,
}
pub type picosplay_tree_t = st_picosplay_tree_t;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
unsafe extern "C" fn splay(mut tree: *mut picosplay_tree_t, mut x: *mut picosplay_node_t) {
    loop {
        let mut p: *mut picosplay_node_t = (*x).parent as *mut picosplay_node_t;
        if p.is_null() {
            (*tree).root = x;
            return;
        }
        let mut g: *mut picosplay_node_t = (*p).parent as *mut picosplay_node_t;
        if (*p).parent.is_null() {
            zig(x);
        } else if x == (*p).left && p == (*g).left || x == (*p).right && p == (*g).right {
            zigzig(x, p);
        } else {
            zigzag(x);
        }
    }
}
unsafe extern "C" fn zig(mut x: *mut picosplay_node_t) {
    rotate(x);
}
unsafe extern "C" fn zigzig(mut x: *mut picosplay_node_t, mut p: *mut picosplay_node_t) {
    rotate(p);
    rotate(x);
}
unsafe extern "C" fn zigzag(mut x: *mut picosplay_node_t) {
    rotate(x);
    rotate(x);
}
#[no_mangle]
pub unsafe extern "C" fn picosplay_init_tree(
    mut tree: *mut picosplay_tree_t,
    mut comp: picosplay_comparator,
    mut create: picosplay_create,
    mut delete_node: picosplay_delete_node,
    mut node_value: picosplay_node_value,
) {
    (*tree).comp = comp;
    (*tree).create = create;
    (*tree).delete_node = delete_node;
    (*tree).node_value = node_value;
    (*tree).root = ::core::ptr::null_mut::<picosplay_node_t>();
    (*tree).size = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn picosplay_new_tree(
    mut comp: picosplay_comparator,
    mut create: picosplay_create,
    mut delete_node: picosplay_delete_node,
    mut node_value: picosplay_node_value,
) -> *mut picosplay_tree_t {
    let mut new: *mut picosplay_tree_t =
        malloc(::core::mem::size_of::<picosplay_tree_t>() as size_t) as *mut picosplay_tree_t;
    if !new.is_null() {
        picosplay_init_tree(new, comp, create, delete_node, node_value);
    }
    return new;
}
#[no_mangle]
pub unsafe extern "C" fn picosplay_insert(
    mut tree: *mut picosplay_tree_t,
    mut value: *mut ::core::ffi::c_void,
) -> *mut picosplay_node_t {
    let mut new: *mut picosplay_node_t = (*tree).create.expect("non-null function pointer")(value);
    if !new.is_null() {
        (*new).left = ::core::ptr::null_mut::<st_picosplay_node_t>();
        (*new).right = ::core::ptr::null_mut::<st_picosplay_node_t>();
        if (*tree).root.is_null() {
            (*tree).root = new;
            (*new).parent = ::core::ptr::null_mut::<st_picosplay_node_t>();
        } else {
            let mut curr: *mut picosplay_node_t = (*tree).root;
            let mut parent: *mut picosplay_node_t = ::core::ptr::null_mut::<picosplay_node_t>();
            let mut left: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while !curr.is_null() {
                parent = curr;
                if (*tree).comp.expect("non-null function pointer")(
                    (*tree).node_value.expect("non-null function pointer")(new),
                    (*tree).node_value.expect("non-null function pointer")(curr),
                ) < 0 as int64_t
                {
                    left = 1 as ::core::ffi::c_int;
                    curr = (*curr).left as *mut picosplay_node_t;
                } else {
                    left = 0 as ::core::ffi::c_int;
                    curr = (*curr).right as *mut picosplay_node_t;
                }
            }
            (*new).parent = parent as *mut st_picosplay_node_t;
            if left != 0 {
                (*parent).left = new as *mut st_picosplay_node_t;
            } else {
                (*parent).right = new as *mut st_picosplay_node_t;
            }
        }
        splay(tree, new);
        (*tree).size += 1;
    }
    return new;
}
#[no_mangle]
pub unsafe extern "C" fn picosplay_find(
    mut tree: *mut picosplay_tree_t,
    mut value: *mut ::core::ffi::c_void,
) -> *mut picosplay_node_t {
    let mut curr: *mut picosplay_node_t = (*tree).root;
    let mut found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !curr.is_null() && found == 0 {
        let mut relation: int64_t = (*tree).comp.expect("non-null function pointer")(
            value,
            (*tree).node_value.expect("non-null function pointer")(curr),
        );
        if relation == 0 as int64_t {
            found = 1 as ::core::ffi::c_int;
        } else if relation < 0 as int64_t {
            curr = (*curr).left as *mut picosplay_node_t;
        } else {
            curr = (*curr).right as *mut picosplay_node_t;
        }
    }
    if !curr.is_null() {
        splay(tree, curr);
    }
    return curr;
}
#[no_mangle]
pub unsafe extern "C" fn picosplay_find_previous(
    mut tree: *mut picosplay_tree_t,
    mut value: *mut ::core::ffi::c_void,
) -> *mut picosplay_node_t {
    let mut curr: *mut picosplay_node_t = (*tree).root;
    let mut previous: *mut picosplay_node_t = ::core::ptr::null_mut::<picosplay_node_t>();
    let mut found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !curr.is_null() && found == 0 {
        let mut relation: int64_t = (*tree).comp.expect("non-null function pointer")(
            value,
            (*tree).node_value.expect("non-null function pointer")(curr),
        );
        if relation == 0 as int64_t {
            found = 1 as ::core::ffi::c_int;
            previous = curr;
        } else if relation < 0 as int64_t {
            curr = (*curr).left as *mut picosplay_node_t;
        } else {
            previous = curr;
            curr = (*curr).right as *mut picosplay_node_t;
        }
    }
    return previous;
}
#[no_mangle]
pub unsafe extern "C" fn picosplay_delete(
    mut tree: *mut picosplay_tree_t,
    mut value: *mut ::core::ffi::c_void,
) {
    let mut node: *mut picosplay_node_t = picosplay_find(tree, value);
    picosplay_delete_hint(tree, node);
}
#[no_mangle]
pub unsafe extern "C" fn picosplay_delete_hint(
    mut tree: *mut picosplay_tree_t,
    mut node: *mut picosplay_node_t,
) {
    if node.is_null() {
        return;
    }
    splay(tree, node);
    if (*node).left.is_null() {
        (*tree).root = (*node).right as *mut picosplay_node_t;
        if !(*tree).root.is_null() {
            (*(*tree).root).parent = ::core::ptr::null_mut::<st_picosplay_node_t>();
        }
    } else if (*node).right.is_null() {
        (*tree).root = (*node).left as *mut picosplay_node_t;
        (*(*tree).root).parent = ::core::ptr::null_mut::<st_picosplay_node_t>();
    } else {
        let mut x: *mut picosplay_node_t = leftmost((*node).right as *mut picosplay_node_t);
        if (*x).parent != node {
            (*(*x).parent).left = (*x).right;
            if !(*x).right.is_null() {
                (*(*x).right).parent = (*x).parent;
            }
            (*x).right = (*node).right;
            (*(*x).right).parent = x as *mut st_picosplay_node_t;
        }
        (*tree).root = x;
        (*x).parent = ::core::ptr::null_mut::<st_picosplay_node_t>();
        (*x).left = (*node).left;
        (*(*x).left).parent = x as *mut st_picosplay_node_t;
    }
    (*tree).delete_node.expect("non-null function pointer")(tree as *mut ::core::ffi::c_void, node);
    (*tree).size -= 1;
}
#[no_mangle]
pub unsafe extern "C" fn picosplay_empty_tree(mut tree: *mut picosplay_tree_t) {
    if !tree.is_null() {
        while !(*tree).root.is_null() {
            picosplay_delete_hint(tree, (*tree).root);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn picosplay_first(mut tree: *mut picosplay_tree_t) -> *mut picosplay_node_t {
    return leftmost((*tree).root);
}
#[no_mangle]
pub unsafe extern "C" fn picosplay_previous(
    mut node: *mut picosplay_node_t,
) -> *mut picosplay_node_t {
    if !(*node).left.is_null() {
        return rightmost((*node).left as *mut picosplay_node_t);
    }
    while !(*node).parent.is_null() && node == (*(*node).parent).left {
        node = (*node).parent as *mut picosplay_node_t;
    }
    return (*node).parent as *mut picosplay_node_t;
}
#[no_mangle]
pub unsafe extern "C" fn picosplay_next(mut node: *mut picosplay_node_t) -> *mut picosplay_node_t {
    if !(*node).right.is_null() {
        return leftmost((*node).right as *mut picosplay_node_t);
    }
    while !(*node).parent.is_null() && node == (*(*node).parent).right {
        node = (*node).parent as *mut picosplay_node_t;
    }
    return (*node).parent as *mut picosplay_node_t;
}
#[no_mangle]
pub unsafe extern "C" fn picosplay_last(mut tree: *mut picosplay_tree_t) -> *mut picosplay_node_t {
    return rightmost((*tree).root);
}
unsafe extern "C" fn rotate(mut child: *mut picosplay_node_t) {
    let mut parent: *mut picosplay_node_t = (*child).parent as *mut picosplay_node_t;
    if (*parent).left == child {
        mark_gp(child);
        (*parent).left = (*child).right;
        if !(*child).right.is_null() {
            (*(*child).right).parent = parent as *mut st_picosplay_node_t;
        }
        (*child).right = parent as *mut st_picosplay_node_t;
    } else {
        mark_gp(child);
        (*parent).right = (*child).left;
        if !(*child).left.is_null() {
            (*(*child).left).parent = parent as *mut st_picosplay_node_t;
        }
        (*child).left = parent as *mut st_picosplay_node_t;
    };
}
unsafe extern "C" fn mark_gp(mut child: *mut picosplay_node_t) {
    let mut parent: *mut picosplay_node_t = (*child).parent as *mut picosplay_node_t;
    let mut grand: *mut picosplay_node_t = (*parent).parent as *mut picosplay_node_t;
    (*child).parent = grand as *mut st_picosplay_node_t;
    (*parent).parent = child as *mut st_picosplay_node_t;
    if grand.is_null() {
        return;
    }
    if (*grand).left == parent {
        (*grand).left = child as *mut st_picosplay_node_t;
    } else {
        (*grand).right = child as *mut st_picosplay_node_t;
    };
}
unsafe extern "C" fn leftmost(mut node: *mut picosplay_node_t) -> *mut picosplay_node_t {
    let mut parent: *mut picosplay_node_t = ::core::ptr::null_mut::<picosplay_node_t>();
    while !node.is_null() {
        parent = node;
        node = (*node).left as *mut picosplay_node_t;
    }
    return parent;
}
unsafe extern "C" fn rightmost(mut node: *mut picosplay_node_t) -> *mut picosplay_node_t {
    let mut parent: *mut picosplay_node_t = ::core::ptr::null_mut::<picosplay_node_t>();
    while !node.is_null() {
        parent = node;
        node = (*node).right as *mut picosplay_node_t;
    }
    return parent;
}
