export default function layoutFormula(goals) {
  return goals.map((g, index) => {
    // populates tiles in a 3x4 grid
    let x = 50 + 430 * ((index) % 3)
    let y = 180 + 200 * ((index) % 4)

    return {
      x,
      y
    }
  })
}
