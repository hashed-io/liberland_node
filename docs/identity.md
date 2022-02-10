Below is a workflow for setting and approving identity attributes

```
# Steve's (pretend user) info
set SEED="bargain album current caught tragic slab identify squirrel embark black drip imitate"
set ADDR="5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym"
```

### Check if Address has an Identity
```
polkadot-js-api --ws wss://n1.hashed.systems query.identity.identityOf 5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym
```
If you see results, run the `clearIdentity` command below

### Clear Identity
```bash
polkadot-js-api --ws wss://n1.hashed.systems tx.identity.clearIdentity --seed "bargain album current caught tragic slab identify squirrel embark black drip imitate"
```

### About Registrars
- The identity pallet has **registrars**, which has the ability to provide Identity **judgements** 
- Citizen1 (`5G3uZjEpvNAQ6U2eUjnMb66B8g6d8wyB68x6CfkRPNcno8eR`) was set by Governance to be a Registrar on this particular chain. 
- A registrar is equivalent to a Minister of Interior, in terms of on-chain duties.
- Check the list of registrars
```bash
polkadot-js-api --ws wss://n1.hashed.systems query.identity.registrars 
```
```json
{
  "registrars": [
    {
      "account": "5ERkMY7QzuBLYegNgKJ3YT8GDuCA3jCgWoRSmbNLaB23rEEQ",
      "fee": "0",
      "fields": []
    },
    {
      "account": "5G3uZjEpvNAQ6U2eUjnMb66B8g6d8wyB68x6CfkRPNcno8eR",
      "fee": "0",
      "fields": []
    }
  ]
}
```

### Call setIdentity  
I believe this **merges** this data with their existing identity data
```bash
polkadot-js-api --ws wss://n1.hashed.systems tx.identity.setIdentity '{"additional": [           
        [
          {
            "Raw": "e-resident"
          },
          {
            "Raw": "1"
          }
        ]
      ]}' --seed "bargain album current caught tragic slab identify squirrel embark black drip imitate"
```

### Query the identity again 
```bash
polkadot-js-api --ws wss://n1.hashed.systems query.identity.identityOf 5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym
```
The applicant has added this information to their profile, notice that there are no judgements 
```json
{
  "identityOf": {
    "judgements": [],
    "deposit": "41,666,666,250",
    "info": {
      "additional": [
        [
          {
            "Raw": "e-resident"
          },
          {
            "Raw": "1"
          }
        ]
      ],
      "display": "None",
      "legal": "None",
      "web": "None",
      "riot": "None",
      "email": "None",
      "pgpFingerprint": null,
      "image": "None",
      "twitter": "None"
    }
  }
}
```

### Applicant calls `requestJudgement` 
Applicant specifies that they are seeking a judgement from a specific Registrar, the Minister of Interior
```bash
polkadot-js-api --ws wss://n1.hashed.systems tx.identity.requestJudgement 1 500 --seed "bargain album current caught tragic slab identify squirrel embark black drip imitate"
```

### Query to see Judgement Awaiting Review
```bash
polkadot-js-api --ws wss://n1.hashed.systems query.identity.identityOf 5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym
```
```json
  "identityOf": {
    "judgements": [
      [
        "1",
        {
          "FeePaid": "0"
        }
      ]
    ],
```

### Minister of Interior calls `provideJudgement` 
Parameters are `RegistrarIndex`, the `AccountId` to judge, and one from a list of possible judgement ratings.
- Fee Paid
- Reasonable
- KnownGood
- OutOfDate
- LowQuality

`KnownGood` indicates fully approved (in this example, as an `e-resident`), although we can likely override the above enum.

```bash
polkadot-js-api --ws wss://n1.hashed.systems tx.identity.provideJudgement 1 5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym KnownGood --seed "exercise museum credit crystal various nature defy human cable any split help"
```

### Check the Identity
The indication that an account is an `e-resident` is that they have a `judgement` with a value of `[1,KnownGood]` and an additional field of `e-resident` set to a value of `1`.

```bash
polkadot-js-api --ws wss://n1.hashed.systems query.identity.identityOf 5HGZfBpqUUqGY7uRCYA6aRwnRHJVhrikn8to31GcfNcifkym
```

```json
{
  "identityOf": {
    "judgements": [
      [
        "1",
        "KnownGood"
      ]
    ],
    "deposit": "41,666,666,250",
    "info": {
      "additional": [
        [
          {
            "Raw": "e-resident"
          },
          {
            "Raw": "1"
          }
        ]
      ],
      "display": "None",
      "legal": "None",
      "web": "None",
      "riot": "None",
      "email": "None",
      "pgpFingerprint": null,
      "image": "None",
      "twitter": "None"
    }
  }
}
```



polkadot-js-api --ws wss://n1.hashed.systems tx.identity.setIdentity '{"display":"Steve Harvey"}' --seed "bargain album current caught tragic slab identify squirrel embark black drip imitate"