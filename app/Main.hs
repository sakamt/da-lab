
import Data.Array.Repa as Repa hiding (map)
import qualified Data.MessagePack as MP
import qualified Data.ByteString.Lazy as B
import Numeric.Odeint
import Numeric.Odeint.Examples

main :: IO ()
main = do
  let v0 = fromListUnboxed (Z :. 3) [1, 0, 0]
  let teo = rk4 (lorenz63 (10, 28, 8.0/3.0)) 0.01
  let ts = take 10000 $ iterate teo v0
  let msg = MP.pack $ map toList ts
  B.writeFile "ts.msg" msg
