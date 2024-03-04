////////////////////////////////////////
/// 
/// シェルピンスキーのギャスケット
/// 
/// /////////////////////////////////////

//マクロ

//定数
const LIVE:bool=true;
const DEAD:bool=false;


fn main() {

   //宣言
   let size =32;
   let mut canvas: [[bool; 32]; 32] = [[true;32];32];
   
   let mut row=0;
   let mut column=0;
   
   for row in 0..size
   {
       canvas[row][0]=DEAD;
   }

   for column in 2..size
   {
       canvas[0][column]=DEAD;
   }
   
   canvas[0][1]=LIVE;

   for row in 0..size-1
   {
       for column in 1..size
       {
           if canvas[row][column]==canvas[row][column-1]
           {
               canvas[row+1][column]=DEAD;
           }
           else
          {
           canvas[row+1][column]=LIVE;
          }
       }
   }

   //出力
   display(canvas);

}

fn display(canvas:[[bool;32];32])
{
   for line in canvas
   {
       for cell in line
       {
           if cell
           {
               print!("■");
           }
           else
           {
               print!("□");
           }
       }
       println!("")
   }
}
