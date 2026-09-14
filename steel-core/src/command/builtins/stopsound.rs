use std::sync::Arc;

use steel_protocol::packets::game::{CStopSound, SoundSource};
use steel_utils::Identifier;

use super::super::{
    brigadier::{CommandNodeBuilder, CommandSyntaxError},
    execution::{
        CommandSource, SteelArgumentType, SteelCommandContext, SteelCommandRuntime, argument,
        literal,
    },
    registration::CommandRegistration,
};

use crate::player::Player;

pub(super) fn registration() -> CommandRegistration<CommandSource> {
    CommandRegistration::new(Identifier::vanilla_static("stopsound"), |_| command())
}

fn command() -> CommandNodeBuilder<CommandSource, SteelCommandRuntime> {
    let mut targets =
        argument("targets", SteelArgumentType::players()).executes(stop_all_for_targets);

    for source in SoundSource::VALUES {
        targets = targets.then(source_command(source));
    }

    targets = targets.then(
        literal("*").executes(move |context| stop_all_for_targets_with_source(context, None)),
    );

    literal("stopsound")
        .executes(stop_all_for_source)
        .then(targets)
}

fn source_command(source: SoundSource) -> CommandNodeBuilder<CommandSource, SteelCommandRuntime> {
    literal(source.name())
        .executes(move |context| stop_all_for_targets_with_source(context, Some(source)))
        .then(
            argument("sound", SteelArgumentType::sound())
                .executes(move |context| stop_sound(context, source)),
        )
}

fn stop_all_for_source(
    context: &SteelCommandContext<CommandSource>,
) -> Result<i32, CommandSyntaxError> {
    let targets = context
        .source()
        .player()
        .map_or_else(Vec::new, |player| vec![Arc::clone(player)]);

    execute(None, &targets)
}

fn stop_all_for_targets(
    context: &SteelCommandContext<CommandSource>,
) -> Result<i32, CommandSyntaxError> {
    let targets = context.players("targets")?;

    execute(None, &targets)
}

fn stop_all_for_targets_with_source(
    context: &SteelCommandContext<CommandSource>,
    source: Option<SoundSource>,
) -> Result<i32, CommandSyntaxError> {
    let targets = context.players("targets")?;

    execute(source, &targets)
}

fn stop_sound(
    context: &SteelCommandContext<CommandSource>,
    source: SoundSource,
) -> Result<i32, CommandSyntaxError> {
    let targets = context.players("targets")?;
    let sound = context.identifier("sound")?.clone();

    execute_sound(source, sound, &targets)
}

fn execute(
    source: Option<SoundSource>,
    targets: &[Arc<Player>],
) -> Result<i32, CommandSyntaxError> {
    for target in targets {
        target.send_packet(CStopSound::new(source, None));
    }

    i32::try_from(targets.len()).map_err(|_| {
        CommandSyntaxError::dynamic("Target player count exceeds the command result range")
    })
}

fn execute_sound(
    source: SoundSource,
    sound: Identifier,
    targets: &[Arc<Player>],
) -> Result<i32, CommandSyntaxError> {
    for target in targets {
        target.send_packet(CStopSound::new(Some(source), Some(sound.clone())));
    }

    i32::try_from(targets.len()).map_err(|_| {
        CommandSyntaxError::dynamic("Target player count exceeds the command result range")
    })
}
