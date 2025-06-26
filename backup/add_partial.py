import json

def main():
    with open("./layouts.json", encoding="utf-8") as f:
        layouts = json.load(f)
    with open("./cached_stats.json", encoding="utf-8") as f:
        cached_stats = json.load(f)
        
    for name, stat in cached_stats.items():
        keys = layouts[name]["keys"]
        sum_ = stat.pop("sum")
        stat_ = stat.pop("stats")
        stat["keys"] = keys
        stat["sum"] = sum_
        stat["stats"] = stat_
    
    with open("./cached_stats2.json", 'w', encoding="utf-8") as f:
        json.dump(cached_stats, f, indent=2)
             
if __name__ == "__main__":
    main()
