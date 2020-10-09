# pysqlcrypto
Allows you to use [sqlcrypto](https://github.com/PizzaCrust/sqlcrypto) crate in python, as a replacement for [pysqlsimplecipher](https://github.com/bssthu/pysqlsimplecipher), to accelerate encryption or decryption by order of magnitudes, decryption is ~7500x faster.

## Install
1. `maturin build`
2. Install the generated wheel file

## Usage
```python
import time
from pysqlcrypto import decrypt, encrypt

timer_start = int(round(time.time(), 0))
with open("database.db", 'rb') as fp:
        raw = fp.read()
with open("database-dec.db", 'wb') as fp:
        fp.write(bytes(decrypt(raw, b'test')))
timer_finish = int(round(time.time(), 0))
timer_total = timer_finish - timer_start
print(str(timer_total) + ' second(s) decrypt.')
```