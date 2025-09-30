# Lucra: Solana Arbitrage Bot

**Lucra** is an advanced Solana-based arbitrage bot designed to optimize cross-platform trading opportunities. With support for multiple decentralized exchanges (DEXs), Lucra leverages powerful strategies such as the **Two-Hop Strategy**, **Triangle Strategy**, and **Multi-DEX Strategy** to maximize profits by exploiting price discrepancies across the ecosystem.

## How it works ? 
The bot automatically executes trades between different DEXs to buy low on one exchange and sell high on another, ensuring maximum profit by reducing slippage and transaction costs and also notifies the user on telegram .

## Architecture 
![architecture](./finalarc.png)

## Alternative architecture 
This architecture has a lot of limitations not suitable for our case .
![Alternative arc](overview.png)


# Hyperliquid Integration ? 

# 1. Cross-Venue Spot-Perpetual Arbitrage

This strategy leverages the advanced capabilities of Hyperliquid's Perpetual Futures platform alongside the spot market liquidity on Solana DEXs to execute high-speed, delta-neutral arbitrage.

---

## The Core Concept: Price Convergence

The strategy exploits temporary price inefficiencies where the spot price of an asset (e.g., SOL) on a Solana DEX differs significantly from the perpetual futures price of the same asset on Hyperliquid.

The theoretical principle is that the perpetual price and the spot price of an asset **must eventually converge**. By simultaneously buying the cheaper asset and selling the more expensive one, the bot locks in a "risk-free" profit, regardless of which direction the market moves afterward.

##  How the Strategy Works ?

The goal is to maintain a **delta-neutral** position, meaning the overall portfolio value is insensitive to the asset's price changes.

### Step 1: Opportunity Detection (The Price Analysis Module)

The bot monitors price feeds from both ecosystems:

* **Solana DEXs:** The actual price of the underlying asset (e.g., SOL) in the spot market.
* **Hyperliquid:** The contract price for the perpetual futures of that asset (e.g., SOL-PERP).

**Scenario: Spot is Cheaper**

$$\text{Price}_{\text{SOL-DEX}} < \text{Price}_{\text{SOL-PERP}}$$

### Step 2: Simultaneous Execution (The Transaction Builder)

The bot executes a trade pair in near-atomic fashion:

1.  **Long Leg (Buy Spot):** **BUY** $\text{SOL}$ on the Solana DEX (e.g., Raydium, Orca) where the price is lower.
2.  **Short Leg (Sell Perpetual):** **SHORT** the $\text{SOL}$ Perpetual contract on Hyperliquid where the price is higher.

### Step 3: Profit Realization (Convergence)

* **Initial Profit Locked:** The difference between the short sale price and the long buy price is the initial gross profit, minus execution and gas fees.
    $$\text{Gross Profit} \approx \text{Quantity} \times (\text{Price}_{\text{PERP}} - \text{Price}_{\text{DEX}})$$
* **Closing the Hedge:** Once the prices on the DEX and Hyperliquid converge (usually quickly due to other arbitrageurs), the bot simultaneously closes both positions:
    * Sell the $\text{SOL}$ on the Solana DEX.
    * Buy back (cover) the $\text{SOL}$ perpetual short on Hyperliquid.

Since the bot is perfectly hedged, the profit is realized from the initial spread, with no exposure to market volatility.

