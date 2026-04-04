import {
  Zap, Shield, Wind, Dumbbell, Brain, Eye, Target, Crosshair,
  Flame, Heart, Crown, Sparkles, Users,
  Hand, Cat, Mountain, Star, Cog, CircleDot
} from "lucide-react";
import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";

interface TraitMeta {
  icon: ReactNode;
  color: string;
  category: "physical" | "technical" | "mental" | "goalkeeper" | "special";
}

const TRAIT_META: Record<string, TraitMeta> = {
  Speedster:          { icon: <Zap className="w-3 h-3" />,            color: "text-cyan-400 bg-cyan-500/10 ring-cyan-500/30",       category: "physical"   },
  Tank:               { icon: <Dumbbell className="w-3 h-3" />,       color: "text-orange-400 bg-orange-500/10 ring-orange-500/30",  category: "physical"   },
  Agile:              { icon: <Wind className="w-3 h-3" />,           color: "text-teal-400 bg-teal-500/10 ring-teal-500/30",        category: "physical"   },
  Tireless:           { icon: <Heart className="w-3 h-3" />,          color: "text-green-400 bg-green-500/10 ring-green-500/30",     category: "physical"   },
  Playmaker:          { icon: <Eye className="w-3 h-3" />,            color: "text-purple-400 bg-purple-500/10 ring-purple-500/30",  category: "technical"  },
  Sharpshooter:       { icon: <Target className="w-3 h-3" />,         color: "text-red-400 bg-red-500/10 ring-red-500/30",           category: "technical"  },
  Dribbler:           { icon: <Sparkles className="w-3 h-3" />,       color: "text-yellow-400 bg-yellow-500/10 ring-yellow-500/30",  category: "technical"  },
  BallWinner:         { icon: <Crosshair className="w-3 h-3" />,      color: "text-amber-400 bg-amber-500/10 ring-amber-500/30",     category: "technical"  },
  Rock:               { icon: <Shield className="w-3 h-3" />,         color: "text-slate-400 bg-slate-500/10 ring-slate-500/30",     category: "technical"  },
  Leader:             { icon: <Crown className="w-3 h-3" />,          color: "text-accent-400 bg-accent-500/10 ring-accent-500/30",  category: "mental"     },
  CoolHead:           { icon: <Brain className="w-3 h-3" />,          color: "text-blue-400 bg-blue-500/10 ring-blue-500/30",        category: "mental"     },
  Visionary:          { icon: <Eye className="w-3 h-3" />,            color: "text-indigo-400 bg-indigo-500/10 ring-indigo-500/30",  category: "mental"     },
  HotHead:            { icon: <Flame className="w-3 h-3" />,          color: "text-red-500 bg-red-500/10 ring-red-500/30",           category: "mental"     },
  TeamPlayer:         { icon: <Users className="w-3 h-3" />,          color: "text-emerald-400 bg-emerald-500/10 ring-emerald-500/30", category: "mental"   },
  SafeHands:          { icon: <Hand className="w-3 h-3" />,           color: "text-sky-400 bg-sky-500/10 ring-sky-500/30",           category: "goalkeeper" },
  CatReflexes:        { icon: <Cat className="w-3 h-3" />,            color: "text-violet-400 bg-violet-500/10 ring-violet-500/30",  category: "goalkeeper" },
  AerialDominance:    { icon: <Mountain className="w-3 h-3" />,       color: "text-sky-400 bg-sky-500/10 ring-sky-500/30",           category: "goalkeeper" },
  CompleteForward:    { icon: <Star className="w-3 h-3" />,           color: "text-accent-400 bg-accent-500/10 ring-accent-500/30",  category: "special"    },
  Engine:             { icon: <Cog className="w-3 h-3" />,            color: "text-primary-400 bg-primary-500/10 ring-primary-500/30", category: "special"  },
  SetPieceSpecialist: { icon: <CircleDot className="w-3 h-3" />,      color: "text-lime-400 bg-lime-500/10 ring-lime-500/30",        category: "special"    },
};

const TRAIT_DETAILED_DESCRIPTIONS: Record<string, string> = {
  EarlyScanner: "Checks surroundings before receiving. Improves first-time actions under pressure",
  BlindSideAwareness: "Detects runners or pressure outside direct vision more often than normal",
  TempoManipulator: "Naturally slows or accelerates play to destabilize shape",
  DelayedPasser: "Waits an extra beat to open a better lane instead of releasing early",
  RiskCalibrator: "Exceptional at choosing when to attempt high-value actions",
  SpaceMagnet: "Finds free pockets unusually well without obvious triggers",
  PressBaiter: "Invites pressure deliberately to free teammates",
  TransitionAnticipator: "Starts moving for the next phase before possession has clearly turned",
  OneTouchSpecialist: "Prefers and executes one-touch combinations at unusual efficiency",
  ToePokeFinisher: "Uses unconventional quick-release finishes in crowded areas",
  OutsideFootPasser: "Uses outside of the boot for angle manipulation",
  DisguisedFirstTouch: "First touch is used to deceive, not just control",
  BounceTechnician: "Unusually good at dealing with awkward bounces or loose balls",
  AerialRedirection: "Redirects headers with intent rather than just winning them",
  HalfVolleyComfort: "Will attempt difficult strikes others avoid",
  RecoveryTouch: "Recovers from poor control unusually well without losing the duel",
  LateBoxArriver: "Times delayed runs into the box unusually well",
  NearPostHunter: "Constantly attacks near-post spaces in crossing situations",
  BlindSideRunner: "Times runs on the defender’s shoulder better than normal",
  DecoyMover: "Makes convincing sacrificial runs that open space for others",
  SecondBallPredator: "Positions for knockdowns and loose clearances unusually well",
  StaticLure: "Stays still longer than expected to disconnect markers before sudden movement",
  ChannelDrifter: "Naturally migrates into half-spaces or channels without explicit instruction",
  FarPostGhost: "Appears at the back post untracked at unusual frequency",
  ReboundInstinct: "Attacks rebound zones immediately after shots more often than average",
  ContainmentSpecialist: "Prefers delaying and guiding over immediate tackling",
  RecoverySprinter: "Keeps composure and line choice during recovery runs instead of just sprinting blindly",
  PassingLaneThief: "Steals interceptions by reading the next pass unusually early",
  BodyAngleManipulator: "Shows attackers into dead zones with unusual consistency",
  TacticalFouler: "Stops transitions cynically but intelligently",
  AerialGrappler: "Uses legal-contact manipulation before aerial duels",
  SecondContactWinner: "Loses first duel but recovers the loose outcome at high rates",
  BigMatchRiser: "Performance lifts under high-stakes conditions",
  BigMatchShrinker: "Drops in initiative or execution under pressure",
  MomentumPlayer: "Confidence swings sharply after a good or bad action",
  ErrorImmunity: "Recovers mentally from mistakes unusually fast",
  CrowdReactive: "Boosted or destabilized by crowd hostility or support",
  Provocable: "More likely to lose discipline when targeted",
  RefereeManipulator: "Adjusts behavior to referee strictness better than average",
  PainMasker: "Sustains output despite knocks better than expected",
  StatusSensitive: "Performs better with leadership responsibility or worse when challenged",
  ClutchExecutor: "Late-game composure rises instead of falls",
  ContactSeller: "Wins fouls by exaggerating contact convincingly",
  ShieldAddict: "Uses body shielding in places where others would release the ball",
  LineStepTrapper: "Manipulates offside timing aggressively",
  QuickRestartOpportunist: "Takes restarts before shape resets",
  TimeKiller: "Protects the ball or slows play intelligently late in matches",
  ChaosCreator: "Prefers actions that destabilize shape even at some loss of clean possession",
  NutmegOpportunist: "Attempts humiliation-based evasion in tight duels",
  BounceRoomDribbler: "Thrives in ugly, ricochet-heavy dribbles rather than clean carries",
  KeeperDisruptor: "Elite at screening, bumping zones, and affecting keeper claims legally",
  ReboundDirector: "Parries into safe zones intentionally",
  CrossPoker: "Prefers fist interventions into specific areas instead of clean claims",
  BreakawayHypnotist: "Delays attacker release through body language",
  LineDictator: "Actively controls defenders rather than just shouting generic commands",
  ThrowLauncher: "Starts counters with unusual release speed and accuracy",
  PenaltyReader: "Reads non-verbal cues rather than diving from historical bias alone",
  TrafficCommander: "Operates unusually well through crowded set-piece traffic",
};

export const ENGINE_TRAIT_NAMES = Object.keys(TRAIT_DETAILED_DESCRIPTIONS).sort();

function sanitizeTraitDescription(text: string): string {
  return text
    .replace(/\bUsually mixed, but can be developed\.?/gi, "")
    .replace(/\bMostly innate\.?/gi, "")
    .replace(/\bRare, mixed\.?/gi, "")
    .replace(/\bRare, innate\.?/gi, "")
    .replace(/\bMixed\.?/gi, "")
    .replace(/\bLearnable\.?/gi, "")
    .replace(/\s{2,}/g, " ")
    .replace(/\s+\./g, ".")
    .trim();
}

function humanizeTraitName(trait: string): string {
  return trait
    .replace(/([a-z])([A-Z])/g, "$1 $2")
    .replace(/\s+/g, " ")
    .trim();
}

export function getTraitMeta(trait: string): (TraitMeta & { label: string; description: string }) | null {
  const meta = TRAIT_META[trait];
  if (!meta) return null;
  return { ...meta, label: trait, description: trait };
}

export function TraitBadge({ trait: traitName, size = "sm" }: { trait: string; size?: "sm" | "xs" }) {
  const { t } = useTranslation();
  const meta = TRAIT_META[traitName];

  const sizeClasses = size === "xs"
    ? "text-[9px] px-1.5 py-0.5 gap-0.5"
    : "text-[10px] px-2 py-0.5 gap-1";
  const label = t(`traits.${traitName}.label`, {
    defaultValue: humanizeTraitName(traitName),
  });
  const defaultDescription =
    TRAIT_DETAILED_DESCRIPTIONS[traitName] ?? humanizeTraitName(traitName);
  const descriptionRaw = t(`traits.${traitName}.desc`, {
    defaultValue: defaultDescription,
  });
  const description = sanitizeTraitDescription(descriptionRaw);
  const fallbackMeta: TraitMeta = {
    icon: <Sparkles className="w-3 h-3" />,
    color: "text-primary-400 bg-primary-500/10 ring-primary-500/30",
    category: "special",
  };
  const resolvedMeta = meta ?? fallbackMeta;

  return (
    <span
      className={`inline-flex items-center font-heading font-bold uppercase tracking-wider rounded-full ring-1 ${resolvedMeta.color} ${sizeClasses}`}
      title={description}
    >
      {resolvedMeta.icon}
      {label}
    </span>
  );
}

export function TraitList({ traits, size = "sm", max }: { traits: string[]; size?: "sm" | "xs"; max?: number }) {
  if (!traits || traits.length === 0) return null;
  const displayed = max ? traits.slice(0, max) : traits;
  const remaining = max && traits.length > max ? traits.length - max : 0;

  return (
    <div className="flex flex-wrap gap-1">
      {displayed.map(t => <TraitBadge key={t} trait={t} size={size} />)}
      {remaining > 0 && (
        <span className="text-[10px] text-gray-500 font-heading self-center">+{remaining}</span>
      )}
    </div>
  );
}

export default TraitBadge;
