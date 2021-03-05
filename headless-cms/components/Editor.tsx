import '@wordpress/block-editor/build-style/style.css'
import '@wordpress/block-library/build-style/style.css'
import '@wordpress/components/build-style/style.css'
import '@wordpress/editor/build-style/style.css'
import '@wordpress/editor/build-style/editor-styles.css'
import '@wordpress/edit-post/build-style/style.css'
import '@wordpress/format-library/build-style/style.css'
import '@wordpress/nux/build-style/style.css'


import { useEffect, useState, Fragment } from '@wordpress/element';
import {
	BlockEditorKeyboardShortcuts,
	BlockEditorProvider,
	BlockList,
	BlockInspector,
	WritingFlow,
	ObserveTyping,
	InspectorControls,
} from '@wordpress/block-editor';
import {
	Popover,
	SlotFillProvider,
	DropZoneProvider,
	TextControl,
	Panel,
	PanelBody,
	PanelRow,
} from '@wordpress/components';
import { registerCoreBlocks } from '@wordpress/block-library';
import { registerBlockType } from "@wordpress/blocks"

import { ProgrammingExercise } from "moocfi-python-editor"

const testData = () => {
	return [
		{
		  "clientId": "a7638e21-329b-47e6-bc78-0fd5dda0875d",
		  "name": "moocfi/exercise",
		  "isValid": true,
		  "attributes": {
			"exerciseId": "a7638e21-329b-47e6-bc78-0fd5dda0875d"
		  },
		  "innerBlocks": []
		},
		{
		  "clientId": "08906def-8b53-41ba-b9d4-f1de7a49e00b",
		  "name": "core/paragraph",
		  "isValid": true,
		  "attributes": {
			"content": "hello",
			"dropCap": false
		  },
		  "innerBlocks": []
		},
		{
		  "clientId": "463825b5-573b-40ab-bb6f-9f2cad723d4b",
		  "name": "moocfi/exercise",
		  "isValid": true,
		  "attributes": {
			"exerciseId": "463825b5-573b-40ab-bb6f-9f2cad723d4b"
		  },
		  "innerBlocks": []
		}
	  ]
}

function Editor() {
	const [ blocks, updateBlocks ] = useState( testData() );
	console.log(JSON.stringify(blocks, undefined, 2))

	useEffect( () => {
		registerCoreBlocks();
		registerBlockType("moocfi/exercise", {
			title: "Exercise",
			description: "Exercise example",
			category: "embed",
			attributes: {
				exerciseId: {
					type: 'string',
					default: "",
				}
			},
			edit: ({setAttributes, clientId}) => {
				setAttributes({ exerciseId: clientId })
				const [exerciseName, setExerciseName] = useState("name");
				const [deadline, setDeadline] = useState("deadline");
				return <div>
					<Fragment>
						<InspectorControls>
							<Panel>
								<PanelBody>
									<PanelRow>
										<TextControl 
											label="Exercise name"
											onChange={
												(val) => {
													setExerciseName(val)
												}
											}
											value={exerciseName}
										/>
										<TextControl 
											label="Deadline"
											onChange={
												(val) => {
													setDeadline(val)
												}
											}
											value={deadline}
										/>
									</PanelRow>
								</PanelBody>
							</Panel>
						</InspectorControls>
						<ProgrammingExercise
							onExerciseDetailsChange={() => {}}
							organization={"test"}
							course={"python-random-testcourse"}
							exercise={exerciseName}
							token={"asd"}
							height={"300px"}
							outputHeight={"auto"}
							outputPosition={"relative"}
							language={"fi"}
						/>
					</Fragment>
			  	</div>;
			},
			save: ({ attributes }) => {
				console.log("Saving")
				return <figure>
					<ProgrammingExercise
						onExerciseDetailsChange={() => {}}
						organization={"test"}
						course={"python-random-testcourse"}
						exercise={attributes['exercise-name']}
						token={"asd"}
						height={"300px"}
						outputHeight={"auto"}
						outputPosition={"relative"}
						language={"fi"}
					/>
				</figure>;
			}
		})
	}, [] );

	return (
		<div className="playground">
			<SlotFillProvider>
				<DropZoneProvider>
					<BlockEditorProvider
						value={ blocks }
						onInput={ () => updateBlocks }
						onChange={ () => updateBlocks }
					>
						<div className="playground__sidebar">
							<BlockInspector />
						</div>
						<div className="editor-styles-wrapper">
							<Popover.Slot />
								<BlockEditorKeyboardShortcuts />
								<WritingFlow>
									<ObserveTyping>
										<BlockList />
									</ObserveTyping>
								</WritingFlow>
							<Popover.Slot />
						</div>
					</BlockEditorProvider>
				</DropZoneProvider>
			</SlotFillProvider>
		</div>
	);
}

export default Editor
