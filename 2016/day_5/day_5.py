import time
import hashlib

idx = 0
key_1, key_2 = "", "________"
start = time.time()
while "_" in key_2:
    m = hashlib.md5()
    m.update("{}{}".format("reyedfim", idx).encode("utf-8"))
    h = m.hexdigest()
    if h.startswith('00000'):
        key_1 += h[5]
        p = int(h[5], 16)
        if p < 8 and key_2[p] == "_":
            key_2 = key_2[:p] + h[6] + key_2[p+1:]
        now = time.time() - start
        print("[{:>7.0f}s|{:7.0f}H/s] {:10} {:32} {:8} {:8}".format(now, idx/now, idx, h, key_2, key_1[:8]))
    idx += 1
