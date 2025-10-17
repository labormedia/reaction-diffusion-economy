# Reaction-Diffusion inspired Economic Model Specification

A self-contained and discrete economic model to continuous reaction-diffusion PDEs providing a clear path for computational realization.

### Formal Specification of the Reaction-Diffusion Economy

#### 1. Primitives
- **Goods**: Let $\mathcal{N} = \{1, 2, \dots, n\}$ be the finite set of all good types (including raw materials, tools, intermediate goods, final goods, and energy capacities).
- **Agents**: Let $\mathcal{I} = \{1, 2, \dots, m\}$ be the finite set of agents.
- **Time Horizon**: Let $\mathcal{T} = \{1, 2, \dots, T\}$ be the finite set of sequential rounds, with $T \in \mathbb{N}$.
- **Initial Endowments**: For each $i \in \mathcal{I}$, let $`\mathbf{e}_i^0 \in \mathbb{R}_{++}^n`$ be the initial endowment vector (strictly positive to ensure interior solutions).
- **Reaction Rules**: Let $\mathcal{R}$ be a finite set of production technologies (reactions). For each $r \in \mathcal{R}$:
  - $`\mathbf{a}_r \in \mathbb{R}_+^n`$: vector of input coefficients (with $`\mathbf{a}_r \neq \mathbf{0}`$).
  - $`\mathbf{b}_r \in \mathbb{R}_+^n`$: vector of output coefficients (with $`\mathbf{b}_r \not\leq \mathbf{a}_r`$ componentwise to ensure positive net production potential).
  These define linear transformation rules: applying intensity $`x_r > 0`$ subtracts $`x_r \mathbf{a}_r`$ from inputs and adds $`x_r \mathbf{b}_r`$ to outputs, with proportions $`\gamma_{r,j} = a_{r,j}`$ (or $`b_{r,j}`$) scaling destruction/creation as in the example (e.g., for inputs $`g_1, g_2, g_3`$ with $`\gamma_1, \gamma_2, \gamma_3`$, output $`g_4`$ is minted at rate $`\min_j (e_j / \gamma_j)`$ scaled by combined $`\gamma`$'s).
- **Preferences**: For each $`i \in \mathcal{I}`$, let $`u_i: \mathbb{R}_+^n \to \mathbb{R}`$ be a continuous, strictly increasing, and strictly quasi-concave utility function (convex preferences: upper contour sets are convex). Strict monotonicity ensures exhaustive use of endowments.
- **Production Feasibility**: The reaction technology for each agent is linear (hence convex feasible set). No external inputs beyond current endowments; transformations are endogenous to the agent's state.

Total endowment conservation holds across phases: $`\sum_{i \in \mathcal{I}} \mathbf{e}_i^{t+1} = \sum_{i \in \mathcal{I}} \mathbf{e}_i^t$ for all $t \in \mathcal{T}`$.

#### 2. Round Dynamics
For each round $t \in \mathcal{T}$, starting from incoming endowments $`\{\mathbf{e}_i^t\}_{i \in \mathcal{I}} \in (\mathbb{R}_+^n)^m`$ (with $`\mathbf{e}_i^1 = \mathbf{e}_i^0`$):

##### Phase 1: Reaction (Endogenous Transformation)
Each agent $`i \in \mathcal{I}`$ independently chooses production intensities $`\mathbf{x}_i^t = (x_i^{t,r})_{r \in \mathcal{R}} \in \mathbb{R}_+^{|\mathcal{R}|}`$ to solve:
$`
\max_{\mathbf{x}_i^t \geq \mathbf{0}} \, u_i\left( \mathbf{e}_i^t - \sum_{r \in \mathcal{R}} x_i^{t,r} \mathbf{a}_r + \sum_{r \in \mathcal{R}} x_i^{t,r} \mathbf{b}_r \right)
`$
subject to the resource constraint:
$`
\sum_{r \in \mathcal{R}} x_i^{t,r} \mathbf{a}_r \leq \mathbf{e}_i^t \quad (\text{componentwise}).
`$
The post-reaction endowment is:
`$
\mathbf{e}_i^{t,\text{react}} = \mathbf{e}_i^t + \sum_{r \in \mathcal{R}} x_i^{t,r} (\mathbf{b}_r - \mathbf{a}_r).
`$
This phase is decentralized and myopic (or anticipatory of Phase 2 equilibrium, as specified in the equilibrium concept below). The net effect allows minting new combinations (e.g., $g_4$) while destroying inputs proportionally, preserving convexity of the production set.

##### Phase 2: Diffusion (Proportional Exchange)
Given $\{\mathbf{e}_i^{t,\text{react}}\}_{i \in \mathcal{I}}$, agents engage in restricted trade via a centralized market clearing:
- Each agent $i$ chooses the exchange fraction $\lambda_i^t \in [0,1]$, committing to sell the proportional bundle $\mathbf{s}_i^t = \lambda_i^t \mathbf{e}_i^{t,\text{react}}$ (direction constrained to span$\{\mathbf{e}_i^{t,\text{react}}\}$).
- Agent $i$ retains the unexchanged portion $(1 - \lambda_i^t) \mathbf{e}_i^{t,\text{react}}$ outside the market.
- The aggregate supply pool is $\mathbf{S}^t = \sum_{i \in \mathcal{I}} \mathbf{s}_i^t = \sum_{i \in \mathcal{I}} \lambda_i^t \mathbf{e}_i^{t,\text{react}} \in \mathbb{R}_+^n$.
- Each agent $i$ receives a demand bundle $\mathbf{z}_i^t \in \mathbb{R}_+^n$ from the pool, where $\{\mathbf{z}_i^t\}_{i \in \mathcal{I}}$ reallocates $\mathbf{S}^t$ exactly:
  $`
  \sum_{i \in \mathcal{I}} \mathbf{z}_i^t = \mathbf{S}^t \quad (\text{componentwise}).
  `$
The outgoing endowment (input to $t+1$) is:
$`
\mathbf{e}_i^{t+1} = (1 - \lambda_i^t) \mathbf{e}_i^{t,\text{react}} + \mathbf{z}_i^t.
`$
Trade is anonymous and global (no spatial structure), with the proportional restriction modeling "diffusion" as constrained bundle trading (agents cannot disaggregate endowments).

#### 3. Equilibrium Concept
A **sequential competitive equilibrium** for the economy is a sequence of allocations, choices, and prices $\{(\{\mathbf{x}_i^t, \lambda_i^t, \mathbf{z}_i^t\}_{i \in \mathcal{I}}, \mathbf{p}^t)\}_{t \in \mathcal{T}}$ satisfying:
- **Feasibility** (for each $t$): The reaction and diffusion constraints in Sections 2a and 2b hold, with $\mathbf{p}^t \in \mathbb{R}_{++}^n$ (normalized, e.g., $\sum_k p_k^t = 1$).
- **Reaction Optimization** (for each $i,t$): $\mathbf{x}_i^t$ maximizes $u_i(\mathbf{e}_i^{t+1})$ over feasible intensities, anticipating the Phase 2 equilibrium given $\mathbf{p}^t$ (subgame perfect: solve backward from diffusion).
- **Diffusion Optimization** (for each $i,t$, given $\mathbf{p}^t$): The pair $(\lambda_i^t, \mathbf{z}_i^t)$ solves
  $`
  \max_{\lambda_i^t \in [0,1], \mathbf{z}_i^t \geq \mathbf{0}} \, u_i\left( (1 - \lambda_i^t) \mathbf{e}_i^{t,\text{react}} + \mathbf{z}_i^t \right)
  `$
  subject to the budget constraint (value of sold bundle funds purchases):
  $`
  \mathbf{p}^t \cdot \mathbf{z}_i^t \leq \lambda_i^t \, (\mathbf{p}^t \cdot \mathbf{e}_i^{t,\text{react}}).
  `$
  (The retained portion is untaxed/unpriced, reflecting the directional constraint.)
- **Market Clearing** (for each $t$, each good $k \in \mathcal{N}$):
  $`
  \sum_{i \in \mathcal{I}} \lambda_i^t e_{i,k}^{t,\text{react}} = \sum_{i \in \mathcal{I}} z_{i,k}^t.
  `$
- **Price Positivity**: $\mathbf{p}^t \gg \mathbf{0}$ (strict to ensure interiority).

#### 4. Properties
- **Convexity**: Feasible sets for reactions (polyhedral, from linear technologies) and preferences (quasi-concave $u_i$) ensure convexity. Diffusion budgets yield convex choice sets.
- **Conservation**: Total endowments evolve as $\sum_i \mathbf{e}_i^{t+1} = \sum_i \mathbf{e}_i^t$ (reactions net-zero in aggregate if balanced, but individual minting/destruction is local; diffusion reallocates without loss).
- **Turing Inspiration**: Reactions mimic activator-inhibitor dynamics (endogenous transformations); diffusion enforces proportional "spreading" (linear scaling), potentially generating patterns (e.g., specialization) over $T$ rounds.
- **Existence**: By standard results (Arrow-Debreu with convexities and restrictions), an equilibrium exists under compactness ($[0,1]$ bounds) and continuity; sequential structure preserves via fixed-point arguments per round.

#### 5. Analytical Mapping (Reaction–Diffusion Analogy and Implementation)

1. **Reaction–Diffusion Analogy**

   * Let $(e_{i,k}^t)$ denote the local concentration of “species” $(k)$ (good $(k)$) at “cell” $(i)$.
   * The **reaction term** corresponds to endogenous transformation:
     $`
     R_{i,k}(\mathbf{e}_i^t) = \sum_{r \in \mathcal{R}} x_i^{t,r}(b_{r,k} - a_{r,k}),
     `$
     which locally produces or consumes goods.
   * The **diffusion term** approximates redistribution:
     $`
     D_{i,k}(\mathbf{e}_i^t) = \sum_{j \neq i} \kappa_{ij} (e_{j,k}^{t,\text{react}} - e_{i,k}^{t,\text{react}}),
     `$
     where $(\kappa_{ij})$ are proportional exchange coefficients induced by the equilibrium allocation $(\lambda_i^t, \mathbf{z}_i^t)$.
     In continuous space, this converges to a Laplacian operator:
     $`
     \frac{\partial e_k}{\partial t} = R_k(\mathbf{e}) + \nabla \cdot (D_k \nabla e_k),
     `$
     linking market reallocation to spatial diffusion.

2. **Potential Function and Energy Analogy**
   If $(u_i(\cdot))$ are differentiable, define aggregate potential
   $`
   \Phi(\{\mathbf{e}_i^t\}) = \sum_i u_i(\mathbf{e}_i^t),
   `$
   which behaves as a Lyapunov function under convex–concave interactions. Diffusion equilibria then correspond to stationary points $(\nabla_{e_{i,k}} \Phi = 0)$, mirroring Turing steady states where marginal utilities align across agents.

3. **Discrete-Time Implementation (Convex Optimization Form)**
   Each round can be computed via a two-stage convex program:

   * **Reaction stage**:
     $`
     \max_{x_i^t} u_i(\mathbf{e}_i^t + A x_i^t), \quad \text{s.t. } A^- x_i^t \le \mathbf{e}_i^t,
     `$
     where $(A = B - A^-)$ encodes reaction net coefficients.
   * **Diffusion stage**:
     $`
     \max_{\lambda_i^t, \mathbf{z}_i^t} u_i((1-\lambda_i^t)\mathbf{e}_i^{t,\text{react}}+\mathbf{z}_i^t)
     `$
     subject to $(\mathbf{p}^t \cdot \mathbf{z}_i^t \le \lambda_i^t (\mathbf{p}^t \cdot \mathbf{e}_i^{t,\text{react}}))$ and market clearing.
     The fixed-point of these subproblems defines $(\mathbf{p}^t, \{\lambda_i^t,\mathbf{z}_i^t\})$.

4. **Equilibrium Stability Criterion**
   Define local perturbations $(\delta \mathbf{e}_i^t)$ and linearize:
   $`
   \delta \mathbf{e}_i^{t+1} = J_R \delta \mathbf{e}_i^t + J_D \delta \mathbf{e}_i^t,
   `$
   where $(J_R)$ and $(J_D)$ are Jacobians of reaction and diffusion maps.
   Stability requires $(\rho(J_R + J_D) < 1)$. If eigenmodes cross unity, endogenous oscillations or specialization patterns emerge—analogous to economic “Turing patterns”.

5. **Computational Notes**

   * Implementable via iterative convex solvers (e.g., *Clarabel* or *OSQP*) for each phase.
   * Suitable for distributed simulation: each agent updates locally using their own $(u_i)$ and reaction rules, exchanging summary prices $(\mathbf{p}^t)$.
   * Supports extensions with stochastic perturbations (random productivity shocks) or spatial kernels for regional economies.
