import os
import json


FINGERS = {
    "LP": 0,
    "LR": 1,
    "LM": 2,
    "LI": 3,
    "LT": 4,
    "RT": 5,
    "RI": 6,
    "RM": 7,
    "RR": 8,
    "RP": 9,
    "TB": 5,
}


def pack_pos(pos: tuple[int, int, int]) -> str:
    row, col, finger = pos
    packed = (row << 8) | (col << 4) | finger
    return hex(packed)[2:].zfill(3)
    
    
def unpack_pos(packed_str: str) -> tuple[int, int, int]:
    packed = int(packed_str, base=16)
    row = packed >> 8 & 0xf
    col = packed >> 4 & 0xf
    finger = packed & 0xf
    return (row, col, finger)
    
def pack_layout(layout: dict[str, dict[str, int | str]]) -> str:
    layout_packed = []
    for key, pos in layout.items():
        row = pos["row"]
        col = pos["col"]
        finger: str = pos["finger"]
        packed_keypos = key + pack_pos((row, col, FINGERS[finger]))
        order = (row << 8) + col
        layout_packed.append((packed_keypos, order))
    layout_packed.sort(key=lambda item: item[1])
    layout_packed = "".join(item[0] for item in layout_packed)
    return layout_packed
    
def main():
    layouts = os.listdir("./layouts")
    grouped = {}
    
    for layout_file in layouts:
        with open(f"./layouts/{layout_file}") as f:
            layout = json.load(f)
        lower_name = os.path.splitext(layout_file)[0]
        name = layout["name"]
        user = layout["user"]
        board = layout["board"]
        keys: dict = layout["keys"]
        
        grouped[lower_name] = {
            "user": user,
            "board": board,
            "keys": pack_layout(keys),
        }

    print(grouped["colemak-dh"])
        
    with open("layouts.json", 'w') as f:
        json.dump(grouped, f, indent=4)
        
        
if __name__ == "__main__":
    main()