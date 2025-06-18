import json

FINGERS = ["LP", "LR", "LM", "LI", "LT", "RT", "RI", "RM", "RR", "RP"]
BAD_RED_MAP = [1, 1, 1, 0, 0, 0, 0, 1, 1, 1]

def main():
    table: dict[str, str] = {}
    def add(combo, metric):
        table[f"{FINGERS[combo[0]]}{FINGERS[combo[1]]}{FINGERS[combo[2]]}"] = metric

    yield_finger_combo = ((i, j, k) for i in range(10) for j in range(10) for k in range(10))
    for combo in yield_finger_combo:
        finger0, finger1, finger2 = combo
        hand0, hand1, hand2 = finger0 >= 5, finger1 >= 5, finger2 >= 5
        if hand0 != hand1 != hand2:
            if finger0 != finger2:
                add(combo, "alt")
            else:
                add(combo, "alt-sfs")
            continue
        if (sf_count := (finger0 == finger1) + (finger1 == finger2)) > 0:
            if sf_count == 1:
                add(combo, "sfb")
            else:
                add(combo, "sft")
            continue
        if hand0 == hand1 == hand2:
            if (roll_to_left := finger0 > finger1 > finger2) or finger0 < finger1 < finger2:
                if roll_to_left == hand0:
                    add(combo, "inoneh")
                else:
                    add(combo, "outoneh")
            else:
                is_sfs = finger0 == finger2
                is_bad = (BAD_RED_MAP[finger0] + BAD_RED_MAP[finger1] + BAD_RED_MAP[finger2]) == 3
                match is_sfs, is_bad:
                    case False, False: add(combo, "red")
                    case False, True: add(combo, "bad-red")
                    case True, False: add(combo, "red-sfs")
                    case True, True: add(combo, "bad-red-sfs")
            continue
        roll_to_left = finger0 > finger1 if hand0 == hand1 else finger1 > finger2
        if roll_to_left == hand1:
            add(combo, "inroll")
        else:
            add(combo, "outroll")
    with open("table.json", 'w') as f:
        json.dump(table, f, indent=4)

if __name__ == "__main__":
    main()