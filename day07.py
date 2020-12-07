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

def walk(visited, graph, target):
    if target not in visited:  # no point in duplicating walks
        visited.add(target)
        for source in graph:
            for child in graph[source]:
                #print("source: {}, child: {}, graph[{}]: {}".format(source, child, source, graph[source]))
                if target == child:
                    # we can also find a path to target using
                    # this bag as source so walk the graph using this as target bag
                    walk(visited, graph, source)

def part1(input):
    TARGET = "shiny gold bag"

    rules = [parse_rule(rule) for rule in input]

    # build a graph of bags from rule set
    bag_graph = defaultdict(set)
    for r in rules:
        for c in r.contains:
            bag_graph[r.bag].add(c.bag)
        if len(r.contains) == 0:
            bag_graph[r.bag] = set()

    # print("bag_graph: {}".format(bag_graph))

    # walk bag graph to find all paths to target
    # track paths taken in visited
    visited = set()
    walk(visited, bag_graph, TARGET)

    # print("part1 visited: {}".format(visited))

    count = len(visited) - 1 # exclude direct path to target
    print("part1 count: {}", count)
    return count

def test_part1_sample():
    input = list( (l.strip() for l in open('./inputs/day07_sample').readlines()) )
    assert part1(input) == 4, "Invalid result - expected 4!"

def test_part1_real():
    input = list( (l.strip() for l in open('./inputs/day07').readlines()) )
    assert part1(input) == 205, "Invalid result!"

#test_parse_rule()
#test_part1_sample()
test_part1_real()
print("Ok!")
