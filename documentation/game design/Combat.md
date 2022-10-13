# Goal
- Skill based (timing)
- attack / defend / counter
- synergy
- items

# UX

# Degraded UX
## 0.1
- click on monster
- if close enough -> hit event
- on monster.hps <= 0 -> monster death event
- on hit event -> update hps
- on death event -> despawn entity

## 0.2
- monster fight back
- monster aggro


# Colliders groups
- player - 1
- monsters - 2
- sword range - 3
- projectile - 4

## Interactions
### Active
player - monster
monster - projectile
monster - sword range
monster - monster

### Ignored
player - projectile
player - sword range