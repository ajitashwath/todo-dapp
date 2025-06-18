# Todo DApp
A decentralized todo list application built on Solana using the Anchor framework. This program allows users to create, manage, and track their todo items on the blockchain.

## 🚀 Features

- User Profile Management: Initialize user profiles to track todo statistics
- Add Todos: Create new todo items with custom content
- Mark/Unmark Todos: Toggle completion status of todo items
- Update Todos: Modify existing todo content
- Remove Todos: Delete todo items and reclaim rent
- Decentralized Storage: All data stored on Solana blockchain

## 📋 Program Overview
The Todo DApp consists of two main account types:

### UserProfile
- Tracks user's todo statistics
- Stores authority (owner) public key
- Maintains counters for total and last todo index

### TodoAccount
- Individual todo item storage
- Contains content, completion status, and metadata
- Associated with specific user authority
