# Regex converter

```bash
# transform this REGEX to the equivalent for Base64 Encoded content (at any offset)
cargo run -r -- "Cookie:( +)pwn"

(?:(?:(?:Q29va2llOiBwd2)[4-7])|(?:(?:(?:(?:(?:[\+/-9A-PR-Za-z](?:(?:[\+/1-9A-DF-TV-Za-jl-z][159BFJNRVZdhlptx]D)|(?:[0EUk][159BFJRVZdhlptx]D)))|(?:Q(?:(?:[\+/13-9A-DF-TV-Za-jl-z][159BFJNRVZdhlptx]D)|(?:2[15BFJNRVZdhlptx]D)|(?:[0EUk][159BFJRVZdhlptx]D))))(?:b2))|(?:Q29Db2)|(?:(?:(?:[\+/-9A-PR-Za-z][0EUk]N)|(?:Q[0EUk]N))(?:Db2)))(?:9ra)(?:(?:(?:WU6ICBwd2)[4-7])|(?:(?:WU6IHB3b)[g-v])))|(?:(?:(?:Q29va2llOiA)|(?:(?:(?:[\+/-9A-PR-Za-z][0EUk]N)|(?:Q[0EUk]N))(?:vb2tpZTo))|(?:(?:(?:(?:(?:[\+/-9A-PR-Za-z](?:(?:[\+/1-9A-DF-TV-Za-jl-z][159BFJNRVZdhlptx]D)|(?:[0EUk][159BFJRVZdhlptx]D)))|(?:Q(?:(?:[\+/13-9A-DF-TV-Za-jl-z][159BFJNRVZdhlptx]D)|(?:2[15BFJNRVZdhlptx]D)|(?:[0EUk][159BFJRVZdhlptx]D))))(?:b2))|(?:Q29Db2)|(?:(?:(?:[\+/-9A-PR-Za-z][0EUk]N)|(?:Q[0EUk]N))(?:Db2)))(?:9raWU6ICA)))(?:gICA)*?(?:(?:gcHdu)|(?:(?:gICBwd2)[4-7])|(?:(?:gIHB3b)[g-v]))))


# transform this REGEX to the equivalent for Base64 Encoded content (at offset multiple of 3)
cargo run -r -- --strict-offset "Cookie:( +)pwn"

(?:(?:Q29va2llOi)(?:AgIC)*?(?:(?:(?:AgIHB3b)[g-v])|(?:(?:Bwd2)[4-7])|(?:AgcHdu)))
```
