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

const SOUND_SOURCES: [SoundSource; 11] = [
    SoundSource::Master,
    SoundSource::Music,
    SoundSource::Records,
    SoundSource::Weather,
    SoundSource::Blocks,
    SoundSource::Hostile,
    SoundSource::Neutral,
    SoundSource::Players,
    SoundSource::Ambient,
    SoundSource::Voice,
    SoundSource::Ui,
];

pub(super) fn registration() -> CommandRegistration<CommandSource> {
    CommandRegistration::new(Identifier::vanilla_static("stopsound"), |_| command())
}

fn command() -> CommandNodeBuilder<CommandSource, SteelCommandRuntime> {
    let mut targets =
        argument("targets", SteelArgumentType::players()).executes(stop_all_for_targets);

    for source in SOUND_SOURCES {
        targets = targets.then(source_command(source));
    }

    literal("stopsound")
        .executes(stop_all_for_source)
        .then(targets)
}

fn source_command(source: SoundSource) -> CommandNodeBuilder<CommandSource, SteelCommandRuntime> {
    literal(source.name())
        .executes(move |context| stop_sound(context, source))
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

    execute(context, None, None, &targets)
}

fn stop_all_for_targets(
    context: &SteelCommandContext<CommandSource>,
) -> Result<i32, CommandSyntaxError> {
    let targets = context.players("targets")?;

    execute(context, None, None, &targets)
}

fn stop_sound(
    context: &SteelCommandContext<CommandSource>,
    source: SoundSource,
) -> Result<i32, CommandSyntaxError> {
    let targets = context.players("targets")?;

    let sound = context.identifier("sound").ok().cloned();

    execute(context, Some(source), sound, &targets)
}

fn execute(
    _context: &SteelCommandContext<CommandSource>,
    source: Option<SoundSource>,
    sound: Option<Identifier>,
    targets: &[Arc<Player>],
) -> Result<i32, CommandSyntaxError> {
    let source = source.map(|source| source.as_varint());

    for target in targets {
        println!("Sending CStopSound to player");

        target.send_packet(CStopSound::new(source, sound.clone()));
    }

    i32::try_from(targets.len()).map_err(|_| {
        CommandSyntaxError::dynamic("Target player count exceeds the command result range")
    })
}
