Git Branching Basics

1. Create a Branch
   git branch <branch-name> # Create a new branch
   git checkout <branch-name> # Switch to the branch
   git checkout -b <branch-name> # Create & switch

2. Merge a Branch
   git checkout main # Switch to target branch (e.g., main)
   git merge <branch-name> # Merge the branch into main

3. Delete a Branch  
   git branch -d <branch-name> # Delete locally (safe, checks merge)
   git branch -D <branch-name> # Force delete (unmerged changes)
   git push origin --delete <branch-name> # Delete remote branch

Create:
git checkout -b new-branch
Merge:
git checkout main && git merge new-branch
Delete:
git branch -d new-branch
