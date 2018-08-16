let i = 0

setInterval(_ => {
  i++
  if (i == 10) {
    let x = {}
    console.log(x.y.z)
  }
  console.log(i)
}, 1000)
