for i in $(seq 1 10)
do
  http GET http://localhost:3000/fwd &
done
wait