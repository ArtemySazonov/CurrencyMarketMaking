project(CurrencyMarketMaking_Report)

find_package(LATEX COMPONENTS PDFLATEX XELATEX)

set(LATEX_OUTPUT_PATH build)
set(OUT_DIRECTORY "${CMAKE_SOURCE_DIR}/${LATEX_OUTPUT_PATH}")
set(WORKINGDIR "${CMAKE_SOURCE_DIR}/report/" )

set(MAIN_TEX_BASE_FILENAME "document")
set(MAIN_TEX "${CMAKE_SOURCE_DIR}/report/${MAIN_TEX_BASE_FILENAME}.tex")
set(MAIN_BIB "${CMAKE_SOURCE_DIR}/report/refs.bib")

set(LATEX_COMPILER ${PDFLATEX_COMPILER})

if(LATEX_FOUND AND LATEX_PDFLATEX_FOUND)
	add_custom_target(latex-prebuild
		COMMAND ${LATEX_COMPILER} --shell-escape -output-directory ${OUT_DIRECTORY} ${MAIN_TEX}
		WORKING_DIRECTORY ${WORKINGDIR}
		DEPENDS ${MAIN_TEX}
    )

	add_custom_target(latex-biblatex
		COMMAND rm -f refs.bib
		COMMAND cp report/refs.bib build/refs.bib
		COMMAND ${BIBTEX_COMPILER} build/document
	)
	add_dependencies(latex-biblatex latex-prebuild)

	add_custom_target(latex-pdf
		COMMAND ${LATEX_COMPILER} --shell-escape -output-directory ${OUT_DIRECTORY} ${MAIN_TEX}
		WORKING_DIRECTORY ${WORKINGDIR}
		DEPENDS ${MAIN_TEX}
	)
	add_dependencies(latex-pdf latex-biblatex latex-prebuild)

	add_custom_target(latex-compile ALL) # Entry point of execution.
	add_dependencies(latex-compile latex-pdf)

else()
	message(ERROR "No latex tools found")
endif()