import { fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { useState } from "react";
import { beforeEach } from "vitest";
import { describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import type { GameStateData, PlayerData, TeamData } from "../store/gameStore";
import PlayerProfile from "./PlayerProfile";

function hasWeeklyWage(text: string, amount: number): boolean {
  const numberPortion = amount.toLocaleString();
  return text.replace(/\s+/g, "").includes(`€${numberPortion}/wk`);
}

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string, params?: any) => {
      if (typeof params === "string") {
        return params;
      }
      const defaultValue =
        params &&
        typeof params === "object" &&
        "defaultValue" in params
          ? params.defaultValue
          : undefined;
      if (key === "common.back") return "Back";
      if (key === "common.contract") return "Contract";
      if (key === "common.renewContract") return "Renew Contract";
      if (key === "common.cancel") return "Cancel";
      if (key === "common.done") return "Done";
      if (key === "common.submit") return "Submit";
      if (key === "common.condition") return "Condition";
      if (key === "common.morale") return "Morale";
      if (key === "common.value") return "Value";
      if (key === "common.wage") return "Wage";
      if (key === "common.age") return "Age";
      if (key === "common.freeAgent") return "Free Agent";
      if (key === "common.unknown") return "Unknown";
      if (key === "finances.perWeekSuffix") return "/wk";
      if (key === "finances.marketValue") return "Market Value";
      if (key === "finances.contractRiskCritical") return "Critical";
      if (key === "finances.contractRiskWarning") return "Warning";
      if (key === "finances.contractRiskStable") return "Stable";
      if (key === "finances.contractExpiresOn")
        return `Expires ${params?.date}`;
      if (key === "playerProfile.contractInfo") return "Contract Info";
      if (key === "playerProfile.dateOfBirth") return "Date of Birth";
      if (key === "playerProfile.weeklyWage") return "Weekly Wage";
      if (key === "playerProfile.noContract") return "No Contract";
      if (key === "playerProfile.yearsRemaining") return "Years Remaining";
      if (key === "playerProfile.contractRisk") return "Contract Risk";
      if (key === "playerProfile.renewalTitle") return "Renew Contract";
      if (key === "playerProfile.renewalWage") return "Offered Wage";
      if (key === "playerProfile.renewalLength") return "Contract Length";
      if (key === "playerProfile.renewalLengthYears")
        return `${params?.count} years`;
      if (key === "playerProfile.renewalSubmit") return "Submit Offer";
      if (key === "playerProfile.renewalBudgetWarning")
        return "Exceeds wage budget";
      if (key === "playerProfile.renewalInvalidWage")
        return "Enter a valid weekly wage";
      if (key === "playerProfile.renewalAccepted") return "Offer accepted";
      if (key === "playerProfile.renewalRejected") return "Offer rejected";
      if (key === "playerProfile.renewalCounter")
        return `Wants more: €${params?.wage}/wk for ${params?.years} years`;
      if (key === "playerProfile.renewalBlocked")
        return "Talks are blocked after your earlier decision";
      if (key === "playerProfile.renewalCooledOff")
        return "Previous talks cooled off, so this starts as a fresh conversation.";
      if (key === "playerProfile.delegateRenewal")
        return "Delegate to Assistant";
      if (key === "playerProfile.renewalDelegateMissingReport")
        return "Assistant report did not include this player.";
      if (key === "playerProfile.renewalConversationTitle")
        return "Negotiation pulse";
      if (key === "playerProfile.renewalProjectionTitle")
        return "Projected financial impact";
      if (key === "playerProfile.renewalProjectionWageBill")
        return `Weekly wage bill ${params?.before} -> ${params?.after}`;
      if (key === "playerProfile.renewalProjectionBudgetUsage")
        return `Wage budget use ${params?.before}% -> ${params?.after}%`;
      if (key === "playerProfile.renewalProjectionRunway")
        return `Cash runway ${params?.before} -> ${params?.after}`;
      if (key === "playerProfile.renewalRound")
        return `Round ${params?.count}`;
      if (key === "playerProfile.renewalPatience") return "Patience";
      if (key === "playerProfile.renewalTension") return "Tension";
      if (key === "playerProfile.renewalFeedbackFirmHeadline")
        return "They want stronger terms before moving.";
      if (key === "playerProfile.renewalFeedbackFirmDetail")
        return "The discussion is still open, but wage level and contract length need to feel clearly worthwhile from their side.";
      if (key === "playerProfile.attributes") return "Attributes";
      if (key === "playerProfile.seasonStats") return "Season Stats";
      if (key === "playerProfile.assistsPer90") return "Assists / 90";
      if (key === "playerProfile.careerHistory") return "Career History";
      if (key === "playerProfile.noCareer") return "No Career";
      if (key === "finances.wagePerWeek") return "Wage/wk";
      return defaultValue ?? key;
    },
    i18n: { language: "en" },
  }),
}));

vi.mock("../utils/backendI18n", () => ({
  resolveBackendText: (
    _key?: string,
    fallback?: string,
    _params?: Record<string, string>,
  ) => fallback ?? "",
}));

vi.mock("../lib/countries", () => ({
  countryName: () => "England",
  isValidCountryCode: () => true,
  normaliseNationality: (value: string) => value,
  resolveCountryFlagCode: () => "GB",
}));

function createTeam(overrides: Partial<TeamData> = {}): TeamData {
  return {
    id: "team-1",
    name: "Alpha FC",
    short_name: "ALP",
    country: "GB",
    city: "London",
    stadium_name: "Alpha Ground",
    stadium_capacity: 30000,
    finance: 500000,
    manager_id: "manager-1",
    reputation: 50,
    wage_budget: 50000,
    transfer_budget: 250000,
    season_income: 0,
    season_expenses: 0,
    formation: "4-4-2",
    play_style: "Balanced",
    training_focus: "General",
    training_intensity: "Balanced",
    training_schedule: "Balanced",
    founded_year: 1900,
    colors: { primary: "#000000", secondary: "#ffffff" },
    starting_xi_ids: [],
    tactical_roles: [],
    form: [],
    history: [],
    ...overrides,
  };
}

function createPlayer(overrides: Partial<PlayerData> = {}): PlayerData {
  return {
    id: "player-1",
    match_name: "J. Smith",
    full_name: "John Smith",
    date_of_birth: "2000-01-01",
    nationality: "GB",
    position: "Forward",
    natural_position: "Forward",
    alternate_positions: [],
    training_focus: null,
    attributes: {
      pace: 60,
      stamina: 60,
      strength: 60,
      agility: 60,
      passing: 60,
      shooting: 60,
      tackling: 60,
      dribbling: 60,
      defending: 60,
      positioning: 60,
      vision: 60,
      decisions: 60,
      composure: 60,
      aggression: 60,
      teamwork: 60,
      leadership: 60,
      handling: 20,
      reflexes: 20,
      aerial: 60,
    },
    condition: 80,
    morale: 75,
    injury: null,
    team_id: "team-1",
    contract_end: "2026-10-15",
    wage: 12000,
    market_value: 350000,
    stats: {
      appearances: 0,
      goals: 0,
      assists: 0,
      clean_sheets: 0,
      yellow_cards: 0,
      red_cards: 0,
      avg_rating: 0,
      minutes_played: 0,
    },
    career: [],
    transfer_listed: false,
    loan_listed: false,
    transfer_offers: [],
    traits: [],
    ...overrides,
  };
}

function createGameState(player: PlayerData): GameStateData {
  return {
    clock: {
      current_date: "2026-08-01T00:00:00Z",
      start_date: "2026-07-01T00:00:00Z",
    },
    manager: {
      id: "manager-1",
      first_name: "Jane",
      last_name: "Doe",
      date_of_birth: "1980-01-01",
      nationality: "GB",
      reputation: 50,
      satisfaction: 50,
      fan_approval: 50,
      team_id: "team-1",
      career_stats: {
        matches_managed: 0,
        wins: 0,
        draws: 0,
        losses: 0,
        trophies: 0,
        best_finish: null,
      },
      career_history: [],
    },
    teams: [createTeam()],
    players: [player],
    staff: [],
    messages: [],
    news: [],
    league: {
      id: "league-1",
      name: "League",
      season: 1,
      fixtures: [],
      standings: [],
    },
    scouting_assignments: [],
    board_objectives: [],
  };
}

function RenewalHarness({ initialPlayer }: { initialPlayer?: PlayerData }) {
  const [gameState, setGameState] = useState<GameStateData>(
    createGameState(initialPlayer ?? createPlayer()),
  );

  return (
    <PlayerProfile
      player={gameState.players[0]}
      gameState={gameState}
      isOwnClub
      onClose={vi.fn()}
      onGameUpdate={setGameState}
    />
  );
}

describe("PlayerProfile contract surfaces", () => {
  beforeEach(() => {
    vi.mocked(invoke).mockReset();
  });

  it("renders expiry date, years remaining, and contract risk for the selected player", () => {
    const player = createPlayer();
    const gameState = createGameState(player);

    render(
      <PlayerProfile
        player={player}
        gameState={gameState}
        isOwnClub
        onClose={vi.fn()}
      />,
    );

    expect(screen.getByText("Contract Info")).toBeInTheDocument();
    expect(screen.getByText("Expires 2026-10-15")).toBeInTheDocument();
    expect(screen.getByText("Years Remaining")).toBeInTheDocument();
    expect(screen.getByText("Contract Risk")).toBeInTheDocument();
    expect(screen.getByText("Critical")).toBeInTheDocument();
    expect(
      screen.getAllByText((_, element) =>
        hasWeeklyWage(element?.textContent ?? "", 230),
      ).length,
    ).toBeGreaterThan(0);
  });

  it("validates renewal offers before submission", async () => {
    vi.mocked(invoke).mockImplementation(
      async (command: string, payload?: any) => {
        if (command === "preview_renewal_financial_impact") {
          const offered = Number(payload?.weeklyWage ?? 0);
          return {
            projection: {
              current_annual_wage_bill: 24000,
              projected_annual_wage_bill: 24000 - 12000 + offered,
              annual_wage_budget: 50000,
              annual_soft_cap: 55000,
              current_weekly_wage_spend: 461,
              projected_weekly_wage_spend: Math.round(
                (24000 - 12000 + offered) / 52,
              ),
              current_cash_runway_weeks: 1084,
              projected_cash_runway_weeks: 500,
              currently_over_budget: false,
              policy_allows: offered <= 55000,
            },
          };
        }

        return createGameState(createPlayer());
      },
    );

    render(<RenewalHarness />);

    fireEvent.click(screen.getByRole("button", { name: "Renew Contract" }));

    fireEvent.change(screen.getByLabelText("Offered Wage"), {
      target: { value: "0" },
    });

    expect(screen.getByText("Enter a valid weekly wage")).toBeInTheDocument();

    fireEvent.change(screen.getByLabelText("Offered Wage"), {
      target: { value: "60000" },
    });

    await waitFor(() => {
      expect(screen.getByText("Exceeds wage budget")).toBeInTheDocument();
      expect(
        screen.getByRole("button", { name: "Submit Offer" }),
      ).toBeDisabled();
      expect(screen.getByText("Projected financial impact")).toBeInTheDocument();
    });
  });

  it("submits a renewal offer and refreshes contract data when accepted", async () => {
    const updatedPlayer = createPlayer({
      contract_end: "2029-08-01",
      wage: 15000,
    });
    const updatedGame = createGameState(updatedPlayer);

    vi.mocked(invoke).mockResolvedValue({
      outcome: "accepted",
      game: updatedGame,
      suggested_wage: null,
      suggested_years: null,
      session_status: "agreed",
      is_terminal: true,
    });

    render(<RenewalHarness />);

    fireEvent.click(screen.getByRole("button", { name: "Renew Contract" }));
    fireEvent.change(screen.getByLabelText("Offered Wage"), {
      target: { value: "15000" },
    });
    fireEvent.change(screen.getByLabelText("Contract Length"), {
      target: { value: "3" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Submit Offer" }));

    await waitFor(() => {
      expect(invoke).toHaveBeenCalledWith("propose_renewal", {
        playerId: "player-1",
        weeklyWage: 15000,
        contractYears: 3,
      });
    });

    await waitFor(() => {
      expect(screen.getByText("Offer accepted")).toBeInTheDocument();
      expect(screen.getByText("Expires 2029-08-01")).toBeInTheDocument();
      expect(
        screen.getAllByText((_, element) =>
          hasWeeklyWage(element?.textContent ?? "", 288),
        ).length,
      ).toBeGreaterThan(0);
      expect(screen.getByText("Stable")).toBeInTheDocument();
      expect(screen.getByRole("button", { name: "Done" })).toBeInTheDocument();
      expect(
        screen.queryByRole("button", { name: "Submit Offer" }),
      ).not.toBeInTheDocument();
    });
  });

  it("shows a rejected state when the renewal offer is turned down", async () => {
    vi.mocked(invoke).mockResolvedValue({
      outcome: "rejected",
      game: createGameState(createPlayer()),
      suggested_wage: null,
      suggested_years: null,
      session_status: "stalled",
      is_terminal: false,
    });

    render(<RenewalHarness />);

    fireEvent.click(screen.getByRole("button", { name: "Renew Contract" }));
    fireEvent.change(screen.getByLabelText("Offered Wage"), {
      target: { value: "12000" },
    });
    fireEvent.change(screen.getByLabelText("Contract Length"), {
      target: { value: "2" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Submit Offer" }));

    await waitFor(() => {
      expect(screen.getByText("Offer rejected")).toBeInTheDocument();
    });
  });

  it("shows improved terms when the player wants more", async () => {
    vi.mocked(invoke).mockResolvedValue({
      outcome: "counter_offer",
      game: createGameState(createPlayer()),
      suggested_wage: 16000,
      suggested_years: 4,
      session_status: "open",
      is_terminal: false,
      feedback: {
        mood: "firm",
        headline_key: "playerProfile.renewalFeedbackFirmHeadline",
        detail_key: "playerProfile.renewalFeedbackFirmDetail",
        tension: 58,
        patience: 64,
        round: 1,
        params: {},
      },
    });

    render(<RenewalHarness />);

    fireEvent.click(screen.getByRole("button", { name: "Renew Contract" }));
    fireEvent.change(screen.getByLabelText("Offered Wage"), {
      target: { value: "13000" },
    });
    fireEvent.change(screen.getByLabelText("Contract Length"), {
      target: { value: "2" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Submit Offer" }));

    await waitFor(() => {
      expect(
        screen.getByText("Wants more: €16000/wk for 4 years"),
      ).toBeInTheDocument();
      expect(screen.getByText("Negotiation pulse")).toBeInTheDocument();
      expect(
        screen.getByText("They want stronger terms before moving."),
      ).toBeInTheDocument();
      expect(screen.getByText("Round 1")).toBeInTheDocument();
      expect(screen.getByText("Patience")).toBeInTheDocument();
      expect(screen.getByText("Tension")).toBeInTheDocument();
    });
  });

  it("shows a cooled-off notice when stale talks reset before a new offer", async () => {
    vi.mocked(invoke).mockResolvedValue({
      outcome: "counter_offer",
      game: createGameState(createPlayer()),
      suggested_wage: 15500,
      suggested_years: 3,
      session_status: "open",
      is_terminal: false,
      cooled_off: true,
      feedback: {
        mood: "calm",
        headline_key: "playerProfile.renewalFeedbackCalmHeadline",
        detail_key: "playerProfile.renewalFeedbackCalmDetail",
        tension: 34,
        patience: 76,
        round: 1,
        params: {},
      },
    });

    render(<RenewalHarness />);

    fireEvent.click(screen.getByRole("button", { name: "Renew Contract" }));
    fireEvent.change(screen.getByLabelText("Offered Wage"), {
      target: { value: "13500" },
    });
    fireEvent.change(screen.getByLabelText("Contract Length"), {
      target: { value: "2" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Submit Offer" }));

    await waitFor(() => {
      expect(
        screen.getByText(
          "Previous talks cooled off, so this starts as a fresh conversation.",
        ),
      ).toBeInTheDocument();
      expect(screen.getByText("Round 1")).toBeInTheDocument();
    });
  });

  it("can delegate a single renewal attempt to the assistant", async () => {
    const delegatedPlayer = createPlayer({
      contract_end: "2029-08-01",
      wage: 14000,
    });
    const updatedGame = createGameState(delegatedPlayer);

    vi.mocked(invoke).mockResolvedValue({
      game: updatedGame,
      report: {
        success_count: 1,
        failure_count: 0,
        stalled_count: 0,
        cases: [
          {
            player_id: "player-1",
            player_name: "John Smith",
            status: "successful",
            agreed_wage: 14000,
            agreed_years: 3,
            note: "I was able to close this one without needing you to step in.",
          },
        ],
      },
    });

    render(<RenewalHarness />);

    fireEvent.click(screen.getByRole("button", { name: "Renew Contract" }));
    fireEvent.click(
      screen.getByRole("button", { name: "Delegate to Assistant" }),
    );

    await waitFor(() => {
      expect(invoke).toHaveBeenCalledWith("delegate_renewals", {
        playerIds: ["player-1"],
        maxWageIncreasePct: 35,
        maxContractYears: 3,
      });
    });

    await waitFor(() => {
      expect(screen.getByText("Offer accepted")).toBeInTheDocument();
      expect(screen.getByRole("button", { name: "Done" })).toBeInTheDocument();
    });
  });

  it("shows a localized error when the assistant report omits the player", async () => {
    vi.mocked(invoke).mockResolvedValue({
      game: createGameState(createPlayer()),
      report: {
        success_count: 0,
        failure_count: 0,
        stalled_count: 0,
        cases: [],
      },
    });

    render(<RenewalHarness />);

    fireEvent.click(screen.getByRole("button", { name: "Renew Contract" }));
    fireEvent.click(
      screen.getByRole("button", { name: "Delegate to Assistant" }),
    );

    await waitFor(() => {
      expect(
        screen.getByText("Assistant report did not include this player."),
      ).toBeInTheDocument();
    });
  });

  it("uses midpoint ranking for tied percentile values", () => {
    const player = createPlayer({
      match_stats: [
        {
          fixture_id: "fixture-1",
          season: 1,
          matchday: 1,
          date: "2026-08-01",
          team_id: "team-1",
          opponent_team_id: "team-2",
          was_home: true,
          minutes_played: 180,
          goals: 0,
          assists: 0,
          shots: 2,
          shots_on_target: 1,
          passes_completed: 20,
          passes_attempted: 24,
          tackles_won: 1,
          interceptions: 1,
          fouls_committed: 0,
          yellow_cards: 0,
          red_cards: 0,
          rating: 6.8,
        },
      ],
    });
    const peerOne = createPlayer({
      id: "player-2",
      team_id: "team-2",
      match_stats: [
        {
          fixture_id: "fixture-2",
          season: 1,
          matchday: 1,
          date: "2026-08-01",
          team_id: "team-2",
          opponent_team_id: "team-1",
          was_home: false,
          minutes_played: 180,
          goals: 0,
          assists: 0,
          shots: 1,
          shots_on_target: 0,
          passes_completed: 18,
          passes_attempted: 22,
          tackles_won: 1,
          interceptions: 0,
          fouls_committed: 0,
          yellow_cards: 0,
          red_cards: 0,
          rating: 6.5,
        },
      ],
    });
    const peerTwo = createPlayer({
      id: "player-3",
      team_id: "team-3",
      match_stats: [
        {
          fixture_id: "fixture-3",
          season: 1,
          matchday: 1,
          date: "2026-08-01",
          team_id: "team-3",
          opponent_team_id: "team-1",
          was_home: false,
          minutes_played: 180,
          goals: 0,
          assists: 0,
          shots: 3,
          shots_on_target: 1,
          passes_completed: 16,
          passes_attempted: 20,
          tackles_won: 0,
          interceptions: 1,
          fouls_committed: 1,
          yellow_cards: 0,
          red_cards: 0,
          rating: 6.4,
        },
      ],
    });
    const gameState = createGameState(player);

    gameState.teams = [
      createTeam(),
      createTeam({ id: "team-2", name: "Beta FC", short_name: "BET" }),
      createTeam({ id: "team-3", name: "Gamma FC", short_name: "GAM" }),
    ];
    gameState.players = [player, peerOne, peerTwo];
    gameState.league = {
      id: "league-1",
      name: "League",
      season: 1,
      fixtures: [],
      standings: [
        {
          team_id: "team-1",
          played: 1,
          won: 1,
          drawn: 0,
          lost: 0,
          goals_for: 1,
          goals_against: 0,
          points: 3,
        },
        {
          team_id: "team-2",
          played: 1,
          won: 0,
          drawn: 1,
          lost: 0,
          goals_for: 0,
          goals_against: 0,
          points: 1,
        },
        {
          team_id: "team-3",
          played: 1,
          won: 0,
          drawn: 0,
          lost: 1,
          goals_for: 0,
          goals_against: 1,
          points: 0,
        },
      ],
    };

    render(
      <PlayerProfile
        player={player}
        gameState={gameState}
        isOwnClub
        onClose={vi.fn()}
      />,
    );

    expect(screen.getByText("Assists / 90")).toBeInTheDocument();
    expect(screen.getAllByText("P50").length).toBeGreaterThan(0);
  });

  it("does not fall back to other positions for percentiles", () => {
    const player = createPlayer({
      match_stats: [
        {
          fixture_id: "fixture-1",
          season: 1,
          matchday: 1,
          date: "2026-08-01",
          team_id: "team-1",
          opponent_team_id: "team-2",
          was_home: true,
          minutes_played: 180,
          goals: 1,
          assists: 0,
          shots: 4,
          shots_on_target: 2,
          passes_completed: 18,
          passes_attempted: 22,
          tackles_won: 1,
          interceptions: 0,
          fouls_committed: 0,
          yellow_cards: 0,
          red_cards: 0,
          rating: 7.1,
        },
      ],
    });
    const midfielderOne = createPlayer({
      id: "player-2",
      team_id: "team-2",
      position: "Midfielder",
      natural_position: "Midfielder",
      match_stats: [
        {
          fixture_id: "fixture-2",
          season: 1,
          matchday: 1,
          date: "2026-08-01",
          team_id: "team-2",
          opponent_team_id: "team-1",
          was_home: false,
          minutes_played: 180,
          goals: 0,
          assists: 1,
          shots: 2,
          shots_on_target: 1,
          passes_completed: 44,
          passes_attempted: 51,
          tackles_won: 3,
          interceptions: 2,
          fouls_committed: 1,
          yellow_cards: 0,
          red_cards: 0,
          rating: 7.0,
        },
      ],
    });
    const midfielderTwo = createPlayer({
      id: "player-3",
      team_id: "team-3",
      position: "Midfielder",
      natural_position: "Midfielder",
      match_stats: [
        {
          fixture_id: "fixture-3",
          season: 1,
          matchday: 1,
          date: "2026-08-01",
          team_id: "team-3",
          opponent_team_id: "team-1",
          was_home: false,
          minutes_played: 180,
          goals: 0,
          assists: 2,
          shots: 3,
          shots_on_target: 1,
          passes_completed: 38,
          passes_attempted: 45,
          tackles_won: 2,
          interceptions: 3,
          fouls_committed: 0,
          yellow_cards: 0,
          red_cards: 0,
          rating: 7.3,
        },
      ],
    });
    const gameState = createGameState(player);

    gameState.teams = [
      createTeam(),
      createTeam({ id: "team-2", name: "Beta FC", short_name: "BET" }),
      createTeam({ id: "team-3", name: "Gamma FC", short_name: "GAM" }),
    ];
    gameState.players = [player, midfielderOne, midfielderTwo];
    gameState.league = {
      id: "league-1",
      name: "League",
      season: 1,
      fixtures: [],
      standings: [
        {
          team_id: "team-1",
          played: 1,
          won: 1,
          drawn: 0,
          lost: 0,
          goals_for: 1,
          goals_against: 0,
          points: 3,
        },
        {
          team_id: "team-2",
          played: 1,
          won: 0,
          drawn: 1,
          lost: 0,
          goals_for: 0,
          goals_against: 0,
          points: 1,
        },
        {
          team_id: "team-3",
          played: 1,
          won: 0,
          drawn: 0,
          lost: 1,
          goals_for: 0,
          goals_against: 1,
          points: 0,
        },
      ],
    };

    render(
      <PlayerProfile
        player={player}
        gameState={gameState}
        isOwnClub
        onClose={vi.fn()}
      />,
    );

    const assistsRow = screen.getByText("Assists / 90").closest("div");
    expect(assistsRow).not.toBeNull();
    expect(within(assistsRow as HTMLElement).getByText("-")).toBeInTheDocument();
  });

  it("shows the current tactical role for a starter in the manager team", () => {
    const player = createPlayer();
    const gameState = createGameState(player);
    gameState.teams[0] = createTeam({
      starting_xi_ids: [
        "player-1",
        "d2",
        "d3",
        "d4",
        "d5",
        "m1",
        "m2",
        "m3",
        "m4",
        "f1",
        "f2",
      ],
      tactical_roles: [
        "LinkForward",
        "FullBackSupport",
        "CenterBackStopper",
        "CenterBackStopper",
        "FullBackSupport",
        "WideProgressor",
        "DeepPlaymaker",
        "BoxToBoxMidfielder",
        "WideProgressor",
        "Poacher",
        "TargetForward",
      ],
    });

    render(
      <PlayerProfile
        player={player}
        gameState={gameState}
        isOwnClub
        onClose={vi.fn()}
      />,
    );

    expect(screen.getByText("Current role")).toBeInTheDocument();
    expect(screen.getByText("Link Forward")).toBeInTheDocument();
    expect(screen.getByText(/Role fit/i)).toBeInTheDocument();
  });

  it("renders badges separately from the dedicated traits section", () => {
    const player = createPlayer({
      badges: ["Sharpshooter", "Playmaker"],
      traits: ["EarlyScanner", "PressBaiter"],
    });

    render(
      <PlayerProfile
        player={player}
        gameState={createGameState(player)}
        isOwnClub
        onClose={vi.fn()}
      />,
    );

    const traitsHeading = screen.getByText("Traits");
    expect(traitsHeading).toBeInTheDocument();
    expect(screen.getByText("Sharpshooter")).toBeInTheDocument();
    expect(screen.getByText("Playmaker")).toBeInTheDocument();
    expect(
      screen.getByTitle(/Checks surroundings before receiving/i),
    ).toBeInTheDocument();
    expect(
      screen.getByTitle(/Invites pressure deliberately to free teammates/i),
    ).toBeInTheDocument();

    const traitsCard = traitsHeading.closest("div")?.parentElement;
    expect(traitsCard).not.toBeNull();
    expect(
      within(traitsCard as HTMLElement).getByTitle(
        /Checks surroundings before receiving/i,
      ),
    ).toBeInTheDocument();
    expect(
      within(traitsCard as HTMLElement).getByTitle(
        /Invites pressure deliberately to free teammates/i,
      ),
    ).toBeInTheDocument();
  });

  it("allows setting a trait development goal", async () => {
    const player = createPlayer({ traits: ["EarlyScanner"] });
    const updatedPlayer = createPlayer({
      traits: ["EarlyScanner"],
      development_plan: {
        trait_goal: {
          target_trait: "DelayedPasser",
          mode: "Learn",
          progress: 0,
          completed_sessions: 0,
        },
        position_goal: null,
      },
    });

    vi.mocked(invoke).mockResolvedValue(createGameState(updatedPlayer));

    render(<RenewalHarness initialPlayer={player} />);
    fireEvent.click(screen.getByRole("button", { name: "Set trait goal" }));

    await waitFor(() => {
      expect(invoke).toHaveBeenCalledWith(
        "set_player_trait_training_goal",
        expect.objectContaining({
          playerId: "player-1",
          mode: "Learn",
        }),
      );
    });
  });
});
