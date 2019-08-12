function component() {
  const element = document.createElement('div')
  element.innerHTML = 'Hello swirld'
  return element
}

document.body.appendChild(component())
