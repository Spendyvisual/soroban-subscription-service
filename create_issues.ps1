$issues = @(
    @{"title"="[Frontend] Initialize React/Vite project"; "body"="**Phase 4**`n`nSetup a new Vite + React + TypeScript project in the `frontend/` directory."}
    @{"title"="[Frontend] Configure ESLint and Prettier"; "body"="**Phase 4**`n`nAdd ESLint and Prettier configurations for strict TypeScript rules."}
    @{"title"="[Frontend] Setup CSS / Styling System"; "body"="**Phase 4**`n`nSetup standard CSS or TailwindCSS based on project aesthetic guidelines."}
    @{"title"="[Frontend] Implement React Router"; "body"="**Phase 4**`n`nAdd react-router-dom and setup routes for Provider Dashboard and Subscriber Portal."}
    @{"title"="[Frontend] Setup global state management"; "body"="**Phase 4**`n`nIntegrate Zustand or React Context for managing global app state."}
    @{"title"="[Frontend] Component: Primary Button"; "body"="**Phase 4**`n`nCreate a reusable Button component with variants (primary, secondary, danger, disabled)."}
    @{"title"="[Frontend] Component: Modal Dialog"; "body"="**Phase 4**`n`nCreate a generic Modal component with focus-trapping and escape-key to close."}
    @{"title"="[Frontend] Component: Toast Notifications"; "body"="**Phase 4**`n`nImplement a toast notification system for success/error alerts."}
    @{"title"="[Frontend] Component: Form Inputs"; "body"="**Phase 4**`n`nCreate reusable Input, Select, and Toggle components with error state styling."}
    @{"title"="[Frontend] Component: Data Card"; "body"="**Phase 4**`n`nCreate a reusable Card component for displaying plans and stats."}
    @{"title"="[Frontend] Wallet: Integrate Freighter SDK"; "body"="**Phase 4**`n`nAdd @stellar/freighter-api and setup basic connect functionality."}
    @{"title"="[Frontend] Wallet: Provider Context"; "body"="**Phase 4**`n`nCreate a React Context to expose the connected wallet address and network state."}
    @{"title"="[Frontend] Wallet: Connect/Disconnect Button"; "body"="**Phase 4**`n`nBuild the UI button in the navbar to connect Freighter and show truncated address."}
    @{"title"="[Frontend] Wallet: Handle Network Mismatch"; "body"="**Phase 4**`n`nShow an error banner if the user is connected to Mainnet instead of Testnet."}
    @{"title"="[Frontend] Wallet: Fetch XLM/USDC Balances"; "body"="**Phase 4**`n`nAdd utility to fetch and display the user's current token balances."}
    @{"title"="[Frontend] Provider UI: Dashboard Layout"; "body"="**Phase 4**`n`nCreate the sidebar and topnav layout specifically for the Provider dashboard view."}
    @{"title"="[Frontend] Provider UI: Create Plan Form"; "body"="**Phase 4**`n`nBuild the form to input plan name, price, interval, and asset type."}
    @{"title"="[Frontend] Provider UI: Plan List Table"; "body"="**Phase 4**`n`nBuild a data table to display all active and inactive plans created by the provider."}
    @{"title"="[Frontend] Provider UI: Revenue Stats Card"; "body"="**Phase 4**`n`nCreate UI to show total revenue, active subscribers, and monthly recurring revenue (MRR)."}
    @{"title"="[Frontend] Provider UI: Plan Detail Modal"; "body"="**Phase 4**`n`nBuild a modal to view specific details of a plan and the edit/deactivate actions."}
    @{"title"="[Frontend] Subscriber UI: Portal Layout"; "body"="**Phase 4**`n`nCreate a distinct layout for end-users browsing and managing their subscriptions."}
    @{"title"="[Frontend] Subscriber UI: Available Plans Grid"; "body"="**Phase 4**`n`nDisplay available subscription plans as a grid of pricing cards."}
    @{"title"="[Frontend] Subscriber UI: Active Subscriptions List"; "body"="**Phase 4**`n`nList the user's currently active subscriptions with next billing date."}
    @{"title"="[Frontend] Subscriber UI: Subscription Detail View"; "body"="**Phase 4**`n`nCreate a view showing subscription status, plan name, and a cancel button."}
    @{"title"="[Frontend] Subscriber UI: Billing History Table"; "body"="**Phase 4**`n`nDisplay a ledger of past charges for a specific subscription."}
    @{"title"="[Frontend] Contract: Fetch Plan Data"; "body"="**Phase 4**`n`nIntegrate soroban-client to fetch plan configurations from the deployed contract."}
    @{"title"="[Frontend] Contract: Fetch User Subscriptions"; "body"="**Phase 4**`n`nFetch all subscription IDs for the connected wallet and hydrate the data."}
    @{"title"="[Frontend] Contract: Submit Subscribe Tx"; "body"="**Phase 4**`n`nWire up the 'Subscribe' button to build and submit the subscribe transaction."}
    @{"title"="[Frontend] Contract: Submit SAC Approve Tx"; "body"="**Phase 4**`n`nWire up the logic to approve the USDC/XLM allowance before subscribing."}
    @{"title"="[Frontend] Contract: Submit Cancel Tx"; "body"="**Phase 4**`n`nWire up the 'Cancel Subscription' button to submit the cancel transaction."}
    @{"title"="[Frontend] UX: Loading Skeletons"; "body"="**Phase 4**`n`nAdd skeleton loaders for the plan grid and tables while data is fetching."}
    @{"title"="[Frontend] UX: Empty States"; "body"="**Phase 4**`n`nDesign and implement empty states for 'No active subscriptions' and 'No plans created'."}
    @{"title"="[Frontend] UX: Transaction Pending Modal"; "body"="**Phase 4**`n`nShow a blocking modal with a spinner while waiting for Freighter to sign and network to confirm."}
    @{"title"="[Frontend] UX: Error Boundaries"; "body"="**Phase 4**`n`nAdd React Error Boundaries to gracefully catch and display UI crashes."}
    @{"title"="[Frontend] UX: Toast on Tx Success/Fail"; "body"="**Phase 4**`n`nTrigger the toast notification component when a contract transaction resolves or fails."}
    @{"title"="[Frontend] Core: Dark/Light Theme Toggle"; "body"="**Phase 4**`n`nImplement theme switching using CSS variables and persist choice in localStorage."}
    @{"title"="[Frontend] Core: Responsive Mobile Nav"; "body"="**Phase 4**`n`nEnsure the dashboard and portal layouts work on mobile devices with a hamburger menu."}
    @{"title"="[Frontend] Utils: Currency Formatter"; "body"="**Phase 4**`n`nCreate utility functions to format stroops to readable XLM/USDC strings."}
    @{"title"="[Frontend] Utils: Date Formatter"; "body"="**Phase 4**`n`nCreate utility functions to format ledger timestamps into localized human-readable dates."}
    @{"title"="[Frontend] Test: Setup Cypress E2E"; "body"="**Phase 4**`n`nInitialize Cypress and write a basic test to verify the app renders without crashing."}
)

foreach ($i in $issues) {
    gh issue create -R Spendyvisual/soroban-subscription-service --title $i.title --body $i.body
    Write-Host "Created issue: $($i.title)"
    Start-Sleep -Milliseconds 500
}
