# Regex converter

```bash
# transform this REGEX to the equivalent for Hexadecimal Encoded content
cargo run -r -- hex "Cookie:( +)pwn"

(?:(?:436f6f6b69653a20)(?:20)*(?:70776e))

# transform this REGEX to the equivalent for Base64 Encoded content
cargo run -r -- base64 "Cookie:( +)pwn"

(?:(?:(?:Q29va2llOiBwd2)[4-7])|(?:Q29va2llOiAgcHdu)|(?:(?:(?:Q[0EUk](?:Nvb))|(?:[\+/-9A-PR-Za-z][0EUk](?:Nvb)))(?:2tpZTogcHdu))|(?:(?:(?:Q29va2llOiAgI)|(?:Q(?:(?:2(?:(?:9Db)|(?:[15BFJNRVZdhlptx](?:Db))))|(?:[0EUk](?:(?:NDb)|(?:[159BFJRVZdhlptx](?:Db))))|(?:[\+/13-9A-DF-TV-Za-jl-z][159BFJNRVZdhlptx](?:Db)))(?:29raWU6I))|(?:[\+/-9A-PR-Za-z](?:(?:[0EUk](?:(?:NDb)|(?:[159BFJRVZdhlptx](?:Db))))|(?:[\+/1-9A-DF-TV-Za-jl-z][159BFJNRVZdhlptx](?:Db)))(?:29raWU6I))|(?:(?:(?:Q[0EUk](?:Nvb))|(?:[\+/-9A-PR-Za-z][0EUk](?:Nvb)))(?:2tpZTogI)))(?:CAgI)*(?:(?:(?:HB3b)[g-v])|(?:(?:CBwd2)[4-7])|(?:CAgcHdu))))
```
