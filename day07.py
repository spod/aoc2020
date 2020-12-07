#! /usr/bin/env 

from collections import defaultdict
from dataclasses import dataclass
from itertools import chain

@dataclass
class BagQty():
    bag: str
    qty: int

@dataclass
class Rule():
    bag: str
    contains: list[BagQty]

def parse_rule(line):
    (bag, rest) = line.split(' contain ')
    bag = bag.replace("bags", "bag")
    contains_str = rest.split(', ')
    # drop trailing period
    contains_str[-1] = contains_str[-1][:-1]
    contains = []
    if contains_str[0] == 'no other bags':
        return Rule(bag=bag, contains=contains)
    for c in contains_str:
        qty = int(c.split(' ')[0])
        b = c.replace("%s " % c.split(' ')[0], "")
        # singular
        b = b.replace("bags", "bag")
        contains.append(BagQty(bag=b, qty=qty))
    return Rule(bag=bag, contains=contains)


def test_parse_rule():
    sample = list( (l.strip() for l in open('./inputs/day07_sample').readlines()) )
    for line in sample:
        print(line, parse_rule(line))

def gold_bags(bag_to_bags):
    TARGET = "shiny gold bag"
    result = set()
    remainder = set(bag_to_bags.keys())
    for bag, contains in bag_to_bags.items():
        if TARGET in contains:
            result.add(bag)
            remainder.discard(bag)
    for bag in remainder:
        if result & bag_to_bags[bag]:
            result.add(bag)

    
    print("gold_bags::result: {}".format(result))
    print("gold_bags::remainder: {}".format(remainder))
    
    return result


def test_gold_bags():
    input = {'light red bag': {'muted yellow bag', 'bright white bag'}, 'dark orange bag': {'muted yellow bag', 'bright white bag'}, 'bright white bag': {'shiny gold bag'}, 'muted yellow bag': {'shiny gold bag', 'faded blue bag'}, 'shiny gold bag': {'vibrant plum bag', 'dark olive bag'}, 'dark olive bag': {'dotted black bag', 'faded blue bag'}, 'vibrant plum bag': {'dotted black bag', 'faded blue bag'}}
    result = {"bright white bag", "muted yellow bag", "dark orange bag", "light red bag"}
    assert gold_bags(input) == result, "Invalid result!"


def part1(input):
    TARGET = "shiny gold bag"
    count = 0
    bag_to_bags = defaultdict(set)

    rules = [parse_rule(rule) for rule in input]

    for r in rules:
        for c in r.contains:
            bag_to_bags[r.bag].add(c.bag)

    print("bag_to_bags: {}".format(bag_to_bags))
    golden_bags = gold_bags(bag_to_bags)
    print("golden_bags: {}".format(golden_bags))

    for r in rules:
        if r.bag in golden_bags:
            count += 1

    print("part1 count: {}".format(count))

    return count

def test_part1_sample():
    input = list( (l.strip() for l in open('./inputs/day07_sample').readlines()) )
    assert part1(input) == 4, "Invalid result - expected 4!"

def test_part1_real():
    input = list( (l.strip() for l in open('./inputs/day07').readlines()) )
    assert part1(input) == 4, "Invalid result!"

#test_parse_rule()
test_part1_sample()
#test_part1_real()
#test_gold_bags()
print("Ok!")
