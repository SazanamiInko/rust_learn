/////////////////////////////////
///
///インクリメント
/// 
//////////////////////////////////

// . メイン関数

fn main() {
    let mut process=0;
 
     println!("現在{}%処理中",process);
     process+=1;
     println!("現在{}%処理中",process);
     process+=10;
     println!("現在{}%処理中",process);
     process-=1;
     println!("現在{}%処理中",process);
     process-=5;
     println!("現在{}%処理中",process);
 }
 