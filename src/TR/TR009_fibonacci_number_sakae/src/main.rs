
/////////////////////////////////
///
/// フィボナッチ数列(栄東入試問題)
/// 
//////////////////////////////////
use rand::Rng;

//メイン関数
fn main() {

    //フィボナッチ数列の10の余りを格納
    //60までのフィボナッチ数列
    let mut fib:Vec<i64>=Vec::new();
    //フィボナッチ数列の下一桁
    let mut fibmod10:Vec<i32>=Vec::new();
    //2で割り切れるフィボナッチの項
    let mut fibpos2:Vec<i32>=Vec::new();
    //5で割り切れるフィボナッチの項
    let mut fibpos5:Vec<i32>=Vec::new();
    //8で割り切れるフィボナッチの項
    let mut fibpos8:Vec<i32>=Vec::new();
    //2で割り切れる項の数
    let mut count2=0;
    //5で割り切れる項の数
    let mut count5=0;
    //現在の項
    let mut pos=0;
    let mut pos8=0;

    //各周期
    let mut route2=0;
    let mut route5=0;
    let mut route8=0;
    let mut route40=0;

    //初項
    fibmod10.push(1);
    fibmod10.push(1);

    fib.push(1);
    fib.push(1);

    //フィボナッチ数の計算
    for n in 2..2023
    {
        //下一桁だけのフィボナッチ数
        let fib_calc=(fibmod10[n-2]+fibmod10[n-1])%10;
        fibmod10.push(fib_calc);
        
        //実際のフィボナッチ数
        if n<51
        {
          fib.push(fib[n-2]+fib[n-1]);
        }
    }

    //集計
    for fib_work in fibmod10
    {
        let mut fib2_work=fib_work;
        let mut fib5_work=fib_work;

        //2で割り切れるフィボナッチ数
        if fib2_work%2==0
        {
            count2=count2+1;
            fibpos2.push(pos);
        }

        //5で割り切れるフィボナッチ数
        if fib5_work%5==0
        {
            count5=count5+1;
            fibpos5.push(pos);
        }
        pos=pos+1;
    }  

    //フィボナッチ数のnの約数には出現の周期性がある。周期性がないな場合はプログラムが誤っている
    let mut gen=rand::thread_rng();
    let mut verify_pos=gen.gen_range(3..100);
    assert_eq!(fibpos2[1]-fibpos2[0],fibpos2[verify_pos+1]-fibpos2[verify_pos]);
    assert_eq!(fibpos5[1]-fibpos5[0],fibpos5[verify_pos+1]-fibpos5[verify_pos]);
    route2=fibpos2[verify_pos+1]-fibpos2[verify_pos];
    route5=fibpos5[verify_pos+1]-fibpos5[verify_pos];

    println!("2023個のフィボナッチ数");
    println!("2でわりきれる数は{}個置きにあります",route2);
    println!("2でわりきれる数は{}個あります",count2);
    println!("5でわりきれる数は{}個置きにあります",route5);
    println!("5でわりきれる数は{}個あります",count5);


     //8にも周期性があるのか
     //8の約数は実数でないとわからないので実際のフィボナッチ数で検証
   for fib_work in fib
   {
        if fib_work%8==0
        {
            fibpos8.push(pos8);
        }
        pos8=pos8+1;
   }

   verify_pos=gen.gen_range(3..fibpos8.len()-1);
   assert_eq!(fibpos8[1]-fibpos8[0],fibpos8[verify_pos+1]-fibpos8[verify_pos]);
   route8=fibpos8[verify_pos+1]-fibpos8[verify_pos];
   println!("8でわりきれる数は{}個置きにあります",fibpos8[verify_pos+1]-fibpos8[verify_pos]);

   println!("40は8*5なので5の周期{}と8の周期{}の最小公倍数ごとに40で割り切れるフィボナッチ数あ現れます",route5,route8);

   route40=get_least_common_multiple(route8,route5);
   println!("{}と{}の最小公倍数は{}です。",route5,route8,route40);
   let div40=2023/route40;
   println!("40でわりきれる数は{}個あります",div40);

}


//最小公倍数を求める
fn get_least_common_multiple(n1:i32,n2:i32)->i32
{
    let mut ans=0;

    if n1==0||n2==0
    {
        panic!("引き数が不正です。");
       
    }

    let g=get_greatest_divisor(n1,n2);
    
    ans=(n1*n2)/g;

    return ans;
}

//最大公約数を求める
fn get_greatest_divisor(n1:i32,n2:i32)->i32
{
    let mut ans=0;
    let mut mod_work=1;

    let mut big=n2;
    let mut min =n1;

    

    if n1==0||n2==0
    {
        panic!("引き数が不正です。");
    }

    if n2<n1
    {
        big=n1;
        min=n2;
    }

    loop
    {
        mod_work=big%min;
        big=min;
        min=mod_work;
        if min==0
        {
            ans=big;
            break;
        }

       
    }

    return ans;

}
