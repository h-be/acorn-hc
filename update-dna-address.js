const path = require('path')
const fs = require('fs')

// overwrite the DNA hash address in the conductor-config
// with the up to date one
const DNA_ADDRESS_FILE = 'profiles_dna_address'
const CONDUCTOR_CONFIG_FILE = 'my-conductor-config.toml'

const dnaAddress = fs.readFileSync(path.join(__dirname, DNA_ADDRESS_FILE))
// read from the local template
const origConductorConfigPath = path.join(__dirname, CONDUCTOR_CONFIG_FILE)
const conductorConfig = fs.readFileSync(origConductorConfigPath).toString()

// replace dna
let newConductorConfig = conductorConfig.replace(
  /hash = '\w+'/g,
  `hash = '${dnaAddress}'`
)

fs.writeFileSync(origConductorConfigPath, newConductorConfig)
