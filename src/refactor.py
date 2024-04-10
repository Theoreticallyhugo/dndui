import os
import subprocess

abilities = [
    "strength",
    "dexterity",
    "constitution",
    "intelligence",
    "wisdom",
    "charisma",
]
skills = [
    "acrobatics",
    "animal_handling",
    "arcana",
    "athletics",
    "deception",
    "history",
    "insight",
    "intimidation",
    "investigation",
    "medicine",
    "nature",
    "perception",
    "performance",
    "persuasion",
    "religion",
    "sleight_of_hand",
    "stealth",
    "survival",
]

stuff = [
    "passive_perception",
    "passive_investigation",
    "passive_insight",
    "proficiency",
    "walking_speed",
    "inspiration",
    "current_hp",
    "max_hp",
    "temp_hp",
    "initiative",
    "armor_class",
    "defenses",
    "conditions",
]

# subprocess.run(
#     args=[f"sed -i '' -e 's/Skill::new()/Skill::new()/g' *"],
#     shell=True,
# )


for i in stuff:
    print(i)
    # proficiency, modifier, Advantage
    subprocess.run(args=[f"sed -i '' -e 's/get_{i}/{i}/g' *"], shell=True)
# subprocess.run(
#     args=["sed -i '' -e 's/acrobatics(/acrobatics(/g' *"], shell=True
# )
# os.system("sed -i '' -e 's/acrobatics(/acrobatics(' *")
