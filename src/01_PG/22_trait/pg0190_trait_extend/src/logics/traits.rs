////////////////////////
/// 
/// トレイット
/// 
/// /////////////////////

///動物
pub trait Animal {
    
    ///歩く
    fn walk(&self)
    {
        println!("とことこ");
    }

    ///鳴く
    fn cry(&self);

    ///種類
    fn introduction(&self);
}

///鳥
pub trait Bird :Animal {

    ///飛ぶ
    fn fly(&self);
    
}