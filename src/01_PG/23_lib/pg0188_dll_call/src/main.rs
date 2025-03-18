//////////////////////////////
/// 
/// RustからRustのdllを呼び出し
/// 
/// ///////////////////////////
use libloading::{Library, Symbol};

type DllFunc = unsafe extern "C" fn(u64, u64) -> u64;

///主関数
fn main() {
    
    unsafe{
        //動的ライブラリの読み込み
        let dll_name="pg0187_dll";
        let dll=Library::new(dll_name)
                .expect("DLLの読み込みに失敗しました");
    
        //関数の取得
        let fn_add:Symbol<DllFunc>=dll.get(b"add") .expect("DLLの取得に失敗しました");
        let fn_minus:Symbol<DllFunc>=dll.get(b"minus") .expect("DLLの取得に失敗しました");
        let fn_multi:Symbol<DllFunc>=dll.get(b"multi") .expect("DLLの取得に失敗しました");
      
        let right=7;
        let left=3;
      
        let add_ans=fn_add(right,left);
        let minus_ans=fn_minus(right,left);
        let multi_ans=fn_multi(right,left);

        println!("{}+{}={}",right,left,add_ans);
        println!("{}-{}={}",right,left,minus_ans);
        println!("{}*{}={}",right,left,multi_ans);
    }
}
