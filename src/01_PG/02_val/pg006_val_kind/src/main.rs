fn main() {
    println!("Hello, world!");/////////////////////////////////
    ///
    ///様々な変数
    /// 
    //////////////////////////////////
    
    /// . メイン関数
    
    fn main() {
        
        //符号あり
        //バイト
        let byte_val:i8=1;
        let neg_byte_val:i8=-1;
        //整数
        let int_val:i32=1;
        let neg_int_val:i32=-1;
        //ロング
        let long_val : i64=1;
        let neg_long_val : i64=--1;
        //数値型
        let decimal_val : i128=1;
        let neg_decimal_val : i128=-1;
    
        //符号なし
        //バイト
        let u_byte_val : u8=1;
        //整数
        let u_int_val : u32=1;
        //ロング
        let u_long_val : u64=1;
        //数値型
        let u_decimal_val : u128=1;
    
    
        //ビルドエラー
        //let neg_u_byte_val : u8=-1;
    
        //浮動小数点
        let floart_val : f64=1.2;
    
    
    
        //表示
        println!("符号あり");
        println!("byte_val={}",byte_val);
        println!("neg_byte_val={}",neg_byte_val);
        println!("int_val={}",int_val);
        println!("neg_int_val={}",neg_int_val);
        println!("long_val={}",long_val);
        println!("neg_long_val={}",neg_long_val);
        println!("decimal_val={}",decimal_val);
        println!("neg_decimal_val={}",neg_decimal_val);
        println!("");
        println!("符号なし");
        println!("u_byte_val={}",u_byte_val);
        println!("u_int_val={}",u_int_val);
        println!("u_long_val={}",u_long_val);
        println!("u_decimal_val={}",u_decimal_val);
        println!("");
        println!("浮動小数点");
        println!("floart_val={}",floart_val);
    
    
    }
    
}
