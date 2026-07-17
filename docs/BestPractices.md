# TechScript 2.0 Best Practices Guide

Recommendations for writing high-performance, secure, and maintainable TechScript applications.

## 1. Minimal Capabilities Principle
Always request the absolute minimum capabilities required by your application to prevent supply chain injection vulnerability elevations.

## 2. Leverage Struct Consts
Whenever structural maps are intended to act as read-only configurations, declare them with `const` or use const-asserts to lock memory representations.

## 3. Cooperative Async Cleanups
Avoid long-running synchronous loops inside async callbacks. Ensure tasks yield execution regularly so the cooperative scheduler event loop can tick.
