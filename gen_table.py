import json

FINGERS = ["LP", "LR", "LM", "LI", "LT", "RT", "RI", "RM", "RR", "RP"]
BAD_RED_MAP = [1, 1, 1, 0, 0, 0, 0, 1, 1, 1]

def main():
    table: dict[str, str] = {}
    def add(finger0, finger1, finger2, metric):
        table[f"{FINGERS[finger0]}{FINGERS[finger1]}{FINGERS[finger2]}"] = metric

    yield_fingers = ((i, j, k) for i in range(10) for j in range(10) for k in range(10))
    for finger0, finger1, finger2 in yield_fingers:
        hand0, hand1, hand2 = finger0 >= 5, finger1 >= 5, finger2 >= 5
        if hand0 != hand1 != hand2:
            if finger0 != finger2:
                add(finger0, finger1, finger2, "alt")
            else:
                add(finger0, finger1, finger2, "alt-sfs")
            continue
        if (sf_count := (finger0 == finger1) + (finger1 == finger2)) > 0:
            if sf_count == 1:
                add(finger0, finger1, finger2, "sfb")
            else:
                add(finger0, finger1, finger2, "sft")
            continue
        if hand0 == hand1 == hand2:
            if (roll_to_left := finger0 > finger1 > finger2) or finger0 < finger1 < finger2:
                if roll_to_left == hand0:
                    add(finger0, finger1, finger2, "inoneh")
                else:
                    add(finger0, finger1, finger2, "outoneh")
            else:
                is_sfs = finger0 == finger2
                is_bad = (BAD_RED_MAP[finger0] + BAD_RED_MAP[finger1] + BAD_RED_MAP[finger2]) == 3
                match is_sfs, is_bad:
                    case False, False: add(finger0, finger1, finger2, "red")
                    case False, True: add(finger0, finger1, finger2, "bad-red")
                    case True, False: add(finger0, finger1, finger2, "red-sfs")
                    case True, True: add(finger0, finger1, finger2, "bad-red-sfs")
            continue
        roll_to_left = finger0 > finger1 if hand0 == hand1 else finger1 > finger2
        if roll_to_left == hand1:
            add(finger0, finger1, finger2, "inroll")
        else:
            add(finger0, finger1, finger2, "outroll")
    with open("table.json", 'w') as f:
        json.dump(table, f, indent=4)

if __name__ == "__main__":
    main()