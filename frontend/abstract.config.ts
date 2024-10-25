import { defineConfig } from '@abstract-money/cli'
import { react, registry, vanilla } from '@abstract-money/cli/plugins'

export default defineConfig({
  out: 'app/_generated/generated-abstract',
  contracts: [
    {
      name: "pop",
      path: "../contracts/pop/schema/abstract",
      namespace: "pop",
      version: "0.1.0",
      moduleType: "app",
    },
  ],
  plugins: [
    react({
      disableAbstractAppFor: ['cw20-base']
    }),
    vanilla({
      enableAbstractAppFor: [
        'pop',
      ]
    }),
    registry({
      contracts: [{
        namespace: 'cw-plus',
        name: 'cw20-base',
        version: '1.0.1'
      }]
  })],
})
