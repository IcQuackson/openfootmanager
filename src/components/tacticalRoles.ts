import type { PlayerData } from "../store/gameStore";
import {
  canonicalPosition,
  translatePositionLabel,
  type PitchSlotRow,
} from "./SquadTab.helpers";

export const ALL_TACTICAL_ROLES = [
  "Goalkeeper",
  "SweeperKeeper",
  "CenterBackStopper",
  "CenterBackCover",
  "CenterBackPlaymaker",
  "FullBackSupport",
  "WingBackAttack",
  "HoldingMidfielder",
  "DeepPlaymaker",
  "BoxToBoxMidfielder",
  "AdvancedPlaymaker",
  "WideProgressor",
  "Poacher",
  "TargetForward",
  "ChannelRunner",
  "LinkForward",
] as const;

export type TacticalRoleId = (typeof ALL_TACTICAL_ROLES)[number];

type Translate = (key: string, options?: { defaultValue?: string }) => string;

const ROLE_LABELS: Record<TacticalRoleId, string> = {
  Goalkeeper: "Goalkeeper",
  SweeperKeeper: "Sweeper Keeper",
  CenterBackStopper: "Stopper",
  CenterBackCover: "Cover Centre-Back",
  CenterBackPlaymaker: "Ball-Playing Centre-Back",
  FullBackSupport: "Full-Back Support",
  WingBackAttack: "Wing-Back Attack",
  HoldingMidfielder: "Holding Midfielder",
  DeepPlaymaker: "Deep Playmaker",
  BoxToBoxMidfielder: "Box-to-Box Midfielder",
  AdvancedPlaymaker: "Advanced Playmaker",
  WideProgressor: "Wide Progressor",
  Poacher: "Poacher",
  TargetForward: "Target Forward",
  ChannelRunner: "Channel Runner",
  LinkForward: "Link Forward",
};

const ROLE_DESCRIPTIONS: Record<TacticalRoleId, string> = {
  Goalkeeper: "Stays home, secures routine actions, and prioritizes safe distribution.",
  SweeperKeeper: "Starts higher, sweeps behind the line, and looks to launch transitions early.",
  CenterBackStopper: "Attacks duels early, steps into contact, and protects the central lane first.",
  CenterBackCover: "Drops off to protect space behind the line and clean up transition danger.",
  CenterBackPlaymaker: "Builds from the back and looks for clean progression passes under pressure.",
  FullBackSupport: "Offers safe width, supports buildup, and recovers into the back line quickly.",
  WingBackAttack: "Pushes high, stretches the flank, and arrives into advanced support zones.",
  HoldingMidfielder: "Holds position in front of the defense and screens counters before they grow.",
  DeepPlaymaker: "Dictates rhythm from deeper zones and connects buildup into midfield progression.",
  BoxToBoxMidfielder: "Covers ground between both boxes and supports transitions in both directions.",
  AdvancedPlaymaker: "Finds pockets between the lines and looks to create the decisive next action.",
  WideProgressor: "Carries and combines down the outside to move the ball into attacking areas.",
  Poacher: "Lives on the last line and attacks finishes rather than buildup touches.",
  TargetForward: "Provides a direct outlet, contests contact, and brings runners into play.",
  ChannelRunner: "Attacks the spaces between defenders and drives transition threat with movement.",
  LinkForward: "Drops toward the ball, combines centrally, and connects midfield to the attack.",
};

const ROLE_ABBREVIATIONS: Record<TacticalRoleId, string> = {
  Goalkeeper: "GK",
  SweeperKeeper: "SK",
  CenterBackStopper: "STP",
  CenterBackCover: "COV",
  CenterBackPlaymaker: "BPD",
  FullBackSupport: "FBS",
  WingBackAttack: "WBA",
  HoldingMidfielder: "HM",
  DeepPlaymaker: "DLP",
  BoxToBoxMidfielder: "B2B",
  AdvancedPlaymaker: "AP",
  WideProgressor: "WP",
  Poacher: "POA",
  TargetForward: "TF",
  ChannelRunner: "CR",
  LinkForward: "LF",
};

export function isTacticalRoleId(value: string): value is TacticalRoleId {
  return (ALL_TACTICAL_ROLES as readonly string[]).includes(value);
}

export function translateTacticalRoleLabel(
  translate: Translate,
  role: string,
): string {
  if (!isTacticalRoleId(role)) return role;
  return translate(`tactics.tacticalRoles.${role}.label`, {
    defaultValue: ROLE_LABELS[role],
  });
}

export function translateTacticalRoleDescription(
  translate: Translate,
  role: string,
): string {
  if (!isTacticalRoleId(role)) return role;
  return translate(`tactics.tacticalRoles.${role}.desc`, {
    defaultValue: ROLE_DESCRIPTIONS[role],
  });
}

export function tacticalRoleAbbreviation(role: string): string {
  if (!isTacticalRoleId(role)) return role.slice(0, 3).toUpperCase();
  return ROLE_ABBREVIATIONS[role];
}

export function tacticalRolesForPitchPosition(position: string): TacticalRoleId[] {
  switch (canonicalPosition(position)) {
    case "Goalkeeper":
      return ["Goalkeeper", "SweeperKeeper"];
    case "LeftBack":
    case "RightBack":
      return ["FullBackSupport", "WingBackAttack"];
    case "LeftWingBack":
    case "RightWingBack":
      return ["WingBackAttack", "FullBackSupport"];
    case "CenterBack":
      return [
        "CenterBackStopper",
        "CenterBackCover",
        "CenterBackPlaymaker",
      ];
    case "DefensiveMidfielder":
      return ["HoldingMidfielder", "DeepPlaymaker", "BoxToBoxMidfielder"];
    case "CentralMidfielder":
      return [
        "DeepPlaymaker",
        "BoxToBoxMidfielder",
        "HoldingMidfielder",
        "AdvancedPlaymaker",
      ];
    case "AttackingMidfielder":
      return ["AdvancedPlaymaker", "LinkForward", "BoxToBoxMidfielder"];
    case "LeftMidfielder":
    case "RightMidfielder":
    case "LeftWinger":
    case "RightWinger":
      return ["WideProgressor", "AdvancedPlaymaker", "ChannelRunner"];
    case "Striker":
      return ["Poacher", "TargetForward", "ChannelRunner", "LinkForward"];
    default:
      return ["BoxToBoxMidfielder"];
  }
}

export function defaultTacticalRoleForPitchPosition(position: string): TacticalRoleId {
  return tacticalRolesForPitchPosition(position)[0];
}

function defenderRoleRow(count: number): TacticalRoleId[] {
  switch (count) {
    case 3:
      return [
        "CenterBackStopper",
        "CenterBackStopper",
        "CenterBackStopper",
      ];
    case 4:
      return [
        "FullBackSupport",
        "CenterBackPlaymaker",
        "CenterBackStopper",
        "FullBackSupport",
      ];
    case 5:
      return [
        "WingBackAttack",
        "CenterBackStopper",
        "CenterBackStopper",
        "CenterBackStopper",
        "WingBackAttack",
      ];
    default:
      return Array.from({ length: count }, () => "CenterBackStopper");
  }
}

function midfieldRoleRow(count: number): TacticalRoleId[] {
  switch (count) {
    case 2:
      return ["DeepPlaymaker", "BoxToBoxMidfielder"];
    case 3:
      return [
        "HoldingMidfielder",
        "BoxToBoxMidfielder",
        "AdvancedPlaymaker",
      ];
    case 4:
      return [
        "WideProgressor",
        "DeepPlaymaker",
        "BoxToBoxMidfielder",
        "WideProgressor",
      ];
    case 5:
      return [
        "WideProgressor",
        "HoldingMidfielder",
        "BoxToBoxMidfielder",
        "AdvancedPlaymaker",
        "WideProgressor",
      ];
    default:
      return Array.from({ length: count }, () => "BoxToBoxMidfielder");
  }
}

function deepMidfieldRoleRow(count: number): TacticalRoleId[] {
  switch (count) {
    case 1:
      return ["HoldingMidfielder"];
    case 2:
      return ["HoldingMidfielder", "DeepPlaymaker"];
    default:
      return Array.from({ length: count }, () => "HoldingMidfielder");
  }
}

function attackingMidfieldRoleRow(count: number): TacticalRoleId[] {
  switch (count) {
    case 1:
      return ["AdvancedPlaymaker"];
    case 2:
      return ["WideProgressor", "AdvancedPlaymaker"];
    case 3:
      return ["WideProgressor", "AdvancedPlaymaker", "WideProgressor"];
    default:
      return Array.from({ length: count }, () => "AdvancedPlaymaker");
  }
}

function forwardRoleRow(count: number): TacticalRoleId[] {
  switch (count) {
    case 1:
      return ["LinkForward"];
    case 2:
      return ["TargetForward", "Poacher"];
    case 3:
      return ["ChannelRunner", "Poacher", "ChannelRunner"];
    default:
      return Array.from({ length: count }, () => "Poacher");
  }
}

export function defaultTacticalRolesForFormation(
  formation: string,
): TacticalRoleId[] {
  const parts = formation
    .split("-")
    .map((part) => Number(part))
    .filter((part) => !Number.isNaN(part));

  if (parts.length === 3) {
    return [
      "Goalkeeper",
      ...defenderRoleRow(parts[0]),
      ...midfieldRoleRow(parts[1]),
      ...forwardRoleRow(parts[2]),
    ];
  }

  if (parts.length === 4) {
    return [
      "Goalkeeper",
      ...defenderRoleRow(parts[0]),
      ...deepMidfieldRoleRow(parts[1]),
      ...attackingMidfieldRoleRow(parts[2]),
      ...forwardRoleRow(parts[3]),
    ];
  }

  return defaultTacticalRolesForFormation("4-4-2");
}

export function resolveTacticalRoles(
  formation: string,
  pitchSlotRows: PitchSlotRow[],
  tacticalRoles?: string[],
): TacticalRoleId[] {
  const slotCount = pitchSlotRows.reduce((count, row) => count + row.slots.length, 0);
  const currentRoles = tacticalRoles ?? [];
  const fallback = defaultTacticalRolesForFormation(formation);

  return Array.from({ length: slotCount }, (_, index) => {
    const current = currentRoles[index];
    return current && isTacticalRoleId(current) ? current : fallback[index];
  });
}

function weightedAverage(
  player: PlayerData,
  weights: Partial<Record<keyof PlayerData["attributes"], number>>,
): number {
  let total = 0;
  let weightTotal = 0;
  for (const [attribute, weight] of Object.entries(weights) as Array<[
    keyof PlayerData["attributes"],
    number,
  ]>) {
    total += (player.attributes[attribute] ?? 50) * weight;
    weightTotal += weight;
  }
  return weightTotal === 0 ? 0 : total / weightTotal;
}

export function tacticalRoleFitScore(player: PlayerData, role: string): number {
  if (!isTacticalRoleId(role)) return 0;

  const score = (() => {
    switch (role) {
      case "Goalkeeper":
        return weightedAverage(player, {
          handling: 0.4,
          reflexes: 0.3,
          composure: 0.15,
          decisions: 0.15,
        });
      case "SweeperKeeper":
        return weightedAverage(player, {
          handling: 0.24,
          reflexes: 0.24,
          passing: 0.18,
          decisions: 0.14,
          composure: 0.1,
          pace: 0.1,
        });
      case "CenterBackStopper":
        return weightedAverage(player, {
          defending: 0.28,
          tackling: 0.22,
          strength: 0.2,
          aggression: 0.12,
          aerial: 0.1,
          positioning: 0.08,
        });
      case "CenterBackCover":
        return weightedAverage(player, {
          defending: 0.24,
          positioning: 0.2,
          pace: 0.16,
          decisions: 0.14,
          composure: 0.14,
          agility: 0.12,
        });
      case "CenterBackPlaymaker":
        return weightedAverage(player, {
          passing: 0.26,
          vision: 0.18,
          composure: 0.16,
          decisions: 0.14,
          defending: 0.14,
          positioning: 0.12,
        });
      case "FullBackSupport":
        return weightedAverage(player, {
          stamina: 0.18,
          defending: 0.18,
          tackling: 0.16,
          passing: 0.14,
          pace: 0.14,
          teamwork: 0.12,
          positioning: 0.08,
        });
      case "WingBackAttack":
        return weightedAverage(player, {
          pace: 0.2,
          stamina: 0.18,
          dribbling: 0.16,
          passing: 0.14,
          agility: 0.12,
          teamwork: 0.1,
          positioning: 0.1,
        });
      case "HoldingMidfielder":
        return weightedAverage(player, {
          positioning: 0.2,
          defending: 0.18,
          tackling: 0.16,
          passing: 0.14,
          decisions: 0.14,
          composure: 0.1,
          teamwork: 0.08,
        });
      case "DeepPlaymaker":
        return weightedAverage(player, {
          passing: 0.24,
          vision: 0.2,
          decisions: 0.16,
          composure: 0.14,
          teamwork: 0.14,
          positioning: 0.12,
        });
      case "BoxToBoxMidfielder":
        return weightedAverage(player, {
          stamina: 0.18,
          teamwork: 0.16,
          passing: 0.14,
          defending: 0.14,
          tackling: 0.12,
          positioning: 0.12,
          decisions: 0.14,
        });
      case "AdvancedPlaymaker":
        return weightedAverage(player, {
          vision: 0.22,
          passing: 0.2,
          decisions: 0.16,
          dribbling: 0.14,
          composure: 0.14,
          teamwork: 0.14,
        });
      case "WideProgressor":
        return weightedAverage(player, {
          pace: 0.18,
          dribbling: 0.2,
          passing: 0.16,
          agility: 0.14,
          decisions: 0.12,
          teamwork: 0.1,
          stamina: 0.1,
        });
      case "Poacher":
        return weightedAverage(player, {
          shooting: 0.24,
          positioning: 0.18,
          pace: 0.16,
          composure: 0.16,
          agility: 0.14,
          decisions: 0.12,
        });
      case "TargetForward":
        return weightedAverage(player, {
          strength: 0.2,
          aerial: 0.18,
          shooting: 0.18,
          composure: 0.14,
          positioning: 0.14,
          teamwork: 0.16,
        });
      case "ChannelRunner":
        return weightedAverage(player, {
          pace: 0.22,
          agility: 0.16,
          positioning: 0.18,
          dribbling: 0.14,
          shooting: 0.14,
          stamina: 0.08,
          decisions: 0.08,
        });
      case "LinkForward":
        return weightedAverage(player, {
          passing: 0.18,
          teamwork: 0.18,
          composure: 0.16,
          strength: 0.12,
          vision: 0.14,
          decisions: 0.12,
          shooting: 0.1,
        });
    }
  })();

  return Math.round(Math.max(1, Math.min(99, score)));
}

export function tacticalRoleFitClassName(score: number): string {
  if (score >= 80) return "text-emerald-600 dark:text-emerald-300";
  if (score >= 70) return "text-green-600 dark:text-green-300";
  if (score >= 60) return "text-lime-600 dark:text-lime-300";
  if (score >= 50) return "text-amber-600 dark:text-amber-300";
  return "text-red-600 dark:text-red-300";
}

export function describePitchSlot(
  translate: Translate,
  slotPosition: string,
): string {
  return translatePositionLabel(translate, slotPosition);
}
