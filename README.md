# Grit
> **Git** • *Noun* - A silly, stupid ~~or~~ and annoying ~~person~~ piece of software
---
> **Grit** • *Noun* - a large amount of perseverance and passion
---
*What one would you choose?* Grit is a rusty rewrite of git, "the stupid content tracker" (to quote it's creator^[[\[1\]]]). While git has a place and will continue to do so, it lacks *modernization*, *friendlyness*, and *usability*. Since it's creation, it continues to be the largest barrier of entry for new programmers, leading to great confusion and many many shouts of "oh no! where did my files go!". Grit aims to improve the listed issues, making git finally fast, friendly, and usable. *Grit improves and adds to git, but remains compatible, despite being a full rewrite*

## Project status

> ⚠️ - we have not implemented... much of anything yet, and builds should be considered pre pre alpha.

> ❔- Git will still manage functionally not implemented to keep remote updated periodically

### Milestones

- [x] And it begins! Project started
- [ ] The first step till salvation. Repository initiated
- [ ] Now your committed! First commit from grit 
- [ ] Finally found your parents! Commits are now chained!
- [ ] But you still need a 🐶! Adding members to the family with `add`
- [ ] Realizing what's missing: Untracked files! 
- [ ] Get your HEAD screwed on!
- [ ] The times are changing, but what exactly? `diff` is finally here
- [ ] Branching out
- [ ] Exploring around (swapping branches)
- [ ] And coming crawling back... Merging back together
- [ ] 🤲🍒 (Cherry Picking)
- [ ] Patching up mistakes: Conflict resolution
- [ ] Turning back the time (Rebase, Revert)
- [ ] Revisiting your commitments (editing commits)
- [ ] And holding on to memories: Stashing
- [ ] Fetching a friend (pull/fetch)
- [ ] And then pushing them away (push)
- [ ] When can I retire? BFG
- [ ] But don't be mistaken, I'm still in the flow! (Git flow)
- [ ] Where has all the time went? Time tracker
- [ ] Sit back and write (git-journal)
- [ ] Everything else *I* feel I'm missing out on
- [ ] Watching the sunset: long term support

## Project Goals

These goals outline what the grit project plans to achieve:

### Usability

When coding grit, we write for usability (but not always accessibility). This means designing output in such a way that it can be interpreted perfectly by anyone, regardless of if it's their first time programming or their 20th year. We take design hints from cargo, Rust's package manager, making use of

- Bold font's
- Color
- Hyperlinks
- Readable explanations
- Justifying content

And other usability enhancing design ques.

We will also be ensuring all of grit is documented.

### Performance

From the start, grit will be coded to be *at least* as fast as git. Rust as a language choice helps us, but we will be leveraging couracurancy and asynchronous operations from the start to distribute the load.

In a different vein, we strive to keep the dependencies on the down low, instead implementing the needed code ourself.

### Extensibility

By design, other rust programs can leverage our strong library, but that's not where we really shine. We dynamically load extensions from [[`/ext`]], and provide easy API's to register new subcommands.

### Feature packed

We aim to achieve 100% git coverage (and then 100% test coverage of that!) but then on *top* of that:

- [ ] time estimates
- [ ] features from BFG repo cleaner done using
- [ ] aliases for git commands (and aliases for `command + flag` combinations) 
- [ ]  sub sub commands (like `grit commit all`)
- [ ] git journal
- [ ] git flow

### Native standardization

We strive to keep everything absolutely consistent across platforms. Everything just... works exactly the same, no matter if your on Linux or Windows.

### Compatibility

An existing git project will always work with grit, and an existing grit project will always work with git. This means you don't need to give anything up to switch to grit and start reeping the rewards.

> **Note:** grit also extends git. This means that it adds features git does not support. This does not mean git compatibility will be broken however, only that you can not all the grit features with git

## Non Goals

We don't try to:

### Replace git

Replacing git is impossible and uneeded. The software works well enough and is near perfect as a standard for us to follow. Removing it would only hurt us

### Change git

While in our minds the git storage system is not perfect, we can not and will not change it. This is because

1. It will break git compatibility
2. It will break outdated repository's

Both of these have workarounds but in the end those workarounds introduce more issues. Git works good enough. 

### Compact

We won't try to be the smallest crate in the world, that would mean compromising on the other goals. We won't even try to be smaller than git, although that would be nice

## Contributing

Open a pull request lol

## Related projects

* gitoxide, another rust git. Idk what's going on there
* 

#### also...

This is like my project after the rust book so it's shitty, everything you read above is like if anything ever happens that's half good

I will give up after about 3 hours work and switch to another side project

Probably.. I wrote this readme to procrastinate and am about to alt tab back to Titanfall
