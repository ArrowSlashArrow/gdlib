import pyperclip, time

inp = ""
prevpaste = pyperclip.paste()
skip_counter = 0

next_skip = 0
def wait_until_new(skip):
    global prevpaste, skip_counter
    while True:
        paste = pyperclip.paste()
        if paste != prevpaste:
            prevpaste = paste
            skip_counter += 1
            
            if skip_counter > skip:
                print(f"detected new paste: {paste}")
                skip_counter = 0
                return
        else:
            time.sleep(0.02)

while True:
    wait_until_new(next_skip)
    inp = prevpaste

    try:
        out_str = ""

        for line in inp.splitlines():
            stripped = line.strip().strip(",")

            if stripped == "json!({":
                out_str += "vec![\n"
                continue
            elif stripped == "});":
                out_str += "];"
                continue

            id, var = stripped.split("\": ")
            out_str += f"    ({id[1:]}, GDValue::Int({var})),\n"

        print(f"pasting string: {out_str}")
        pyperclip.copy(out_str)
        next_skip = 1

    except:
        print(f"failed to refactor: {inp}")
        next_skip = 0